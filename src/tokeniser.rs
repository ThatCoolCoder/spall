// WIP new tokeniser that actually parses things well.
// The previous implementation used iterators and lambdas but I've decided to use plain for-loops this time,
// as the iterators became too complex when implementing complex patterns

use std::collections::HashMap;

use derive_more::Display;

use crate::javascript_type::JavascriptType;
use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;

// Root token class
pub enum Token {
    Tag(TagToken),
    Content(ContentToken),
    InlineJavascript(InlineJavascriptToken),
}

// Represents a single html tag - <opening>, </closing> or <standalone />
#[derive(Display)]
#[display(fmt = "[{tag_type} {name} tag]")]
pub struct TagToken {
    pub name: String,
    pub attributes: Vec<TagAttribute>, // eg style or id
    pub tag_type: TagType,
}
// Represents (content) inner text of a html node.
#[derive(Display)]
#[display(fmt = "[Content: {value}]")]
pub struct ContentToken {
    pub value: String,
}
// Represents a chunk of javascript found in the markup
#[derive(Display)]
#[display(fmt = "[{javascript_type} inline javascript: {value}]")]
pub struct InlineJavascriptToken {
    pub value: String,
    pub javascript_type: JavascriptType,
}

pub fn read_element(markup: &str) -> Vec<Token> {
    let mut remaining = markup.to_string();
    let mut inside_script_tag = false;
    let mut result = vec![];
    while remaining.len() > 0 {
        // Read tag
        if remaining.chars().next().unwrap() == '<' {
            println!("open tag");
            let (tag, chars) = read_html_tag(&remaining);
            remaining.drain(chars..);
            inside_script_tag = tag.name == "<script>" && tag.tag_type == TagType::Start;
            println!("{}", chars);
            result.push(Token::Tag(tag));
        }
        // Read inline javascript
        else if remaining.chars().next().unwrap() == '~' {
            println!("tilde");
            let (inline_js, size) = read_inline_javascript(&remaining);
            remaining.drain(..size);
            result.push(Token::InlineJavascript(inline_js));
        }
        // Read script tag content
        else if inside_script_tag {
            println!("script");
            let js = read_javascript(markup);
            remaining.drain(..js.len());
            result.push(Token::Content(ContentToken { value: js }));
        }
        // Read normal tag content
        else {
            println!("content");
            let content = read_tag_content(markup);
            remaining.drain(..content.len());
            result.push(Token::Content(ContentToken { value: content }));
        }
        break;
    }
    return result;
}

fn read_html_tag(markup: &str) -> (TagToken, usize) {
    // Read an open/close/standalone tag. Second return value is tag length

    let mut tag_name = "".to_string();
    let mut idx = 1; // start at 1 to skip the opening "<"
    let mut tag_type = TagType::Start;

    // Read tag name
    while idx < markup.len() {
        let char = get_char_unwrap(markup, idx);
        idx += 1;
        match char {
            ' ' | '>' => {
                break;
            }
            '/' => {
                if idx == 1 {
                    tag_type = TagType::End;
                } else {
                    tag_type = TagType::Standalone;
                }
            }
            _ => {
                tag_name.push(char);
            }
        }
    }

    println!("I read a tag called {tag_name} and it was {idx} chars long");

    // Read tag attributes
    let mut tag_attributes = vec![];
    while idx < markup.len() {
        // Read until equals sign
        let mut char = get_char_unwrap(markup, idx);

        if char == '>' {
            break;
        }
        let mut attribute_name = "".to_string();

        while char != '=' && idx < markup.len() {
            attribute_name.push(char);
            char = get_char_unwrap(markup, idx);
            idx += 1;
        }
        idx += 1; // jump past equals sign
        if idx >= markup.len() {
            idx = markup.len() - 1;
            break;
        }

        // Read attribute value
        let mut attribute_value = read_string('"', '\\', &markup[idx..]);
        idx += attribute_value.len();

        // Remove quotes
        attribute_value.pop();
        attribute_value.remove(0);

        tag_attributes.push(TagAttribute {
            name: attribute_name,
            value: attribute_value,
        })
    }

    return (
        TagToken {
            name: tag_name,
            attributes: tag_attributes,
            tag_type: tag_type,
        },
        idx,
    );
}

fn get_char_unwrap(data: &str, idx: usize) -> char {
    return data.chars().nth(idx).unwrap();
}

fn read_tag_content(markup: &str) -> String {
    // Read the content (inner text) of a tag.
    let mut result = "".to_string();
    for char in markup.chars() {
        if char == '<' {
            break;
        }
        result.push(char);
    }
    return result;
}

fn read_javascript(data: &str) -> String {
    // Read some JavaScript until it is ended by a </script> tag.
    // Returned value does not include the ending script tag

    // todo: when I have internet get a library to make this a const
    let brackets: HashMap<char, bool> = [
        ('(', true),
        (')', false),
        ('[', true),
        (']', false),
        ('{', true),
        ('}', false),
    ]
    .iter()
    .cloned()
    .collect();
    let quotes = vec!['\'', '`', '"'];

    let mut result = "".to_string();
    let mut bracket_stack = vec![];
    // let mut bracket_stack = vec![];
    let mut char_idx = 0;
    for char in data.chars() {
        match brackets.get(&char) {
            Some(is_opening) => {
                if *is_opening {
                    bracket_stack.push(char);
                } else {
                    bracket_stack.pop();
                }
            }
            None => (),
        }

        if quotes.contains(&char) {
            let string = read_string(char, '\\', &data[char_idx..]);
            result += &string;
            char_idx += string.len();
        } else {
            result.push(char);
            char_idx += 1;
        }
        if bracket_stack.len() == 0 && &data[char_idx..] == "</script>" {
            break;
        }
    }
    return result;
}

fn read_inline_javascript(markup: &str) -> (InlineJavascriptToken, usize) {
    // Read inline javascript.
    // Expects markup to begin with a tilde (inline start char)

    let mut result = "".to_string();
    for char in markup.chars().skip(1) {
        result.push(char);
        if char == '~' || char == '\n' {
            break;
        }
    }
    let javascript_type = find_javascript_type(&result);
    return (
        InlineJavascriptToken {
            value: result.clone(),
            javascript_type: javascript_type,
        },
        result.len(),
    );
}

fn read_string(quote_char: char, escape_char: char, data: &str) -> String {
    // Generic method for reading a string.
    // Presumes data starts with quote char, returned data includes start and end quote

    let mut result = "".to_string();
    let mut quotes_found = 0;
    let mut last_char_is_escape = false;
    for char in data.chars() {
        result.push(char);

        if char == quote_char && !last_char_is_escape {
            quotes_found += 1;
        }
        // 2 quotes = 1 start, 1 end
        if quotes_found == 2 {
            break;
        }

        if char == escape_char {
            last_char_is_escape = true;
        }
    }
    return result;
}

fn find_tag_type(tag_content: &str) -> TagType {
    // Tag content should not include angle brackets

    // </tag>
    if tag_content.chars().skip(1).next().unwrap() == '/' {
        return TagType::End;
    }
    // <tag />
    else if tag_content.chars().last().unwrap() == '/' {
        return TagType::Standalone;
    }
    // <tag>
    else {
        return TagType::Start;
    }
}

fn find_javascript_type(javascript: &str) -> JavascriptType {
    if javascript.chars().last().unwrap() == '{' {
        return JavascriptType::BlockStart;
    } else if javascript.chars().next().unwrap() == '}' {
        return JavascriptType::BlockEnd;
    } else {
        return JavascriptType::Standalone;
    }
}
