// WIP new tokeniser that actually parses things well.
// I've decided to not use iterators since that doesn't scale well

use crate::javascript_type::JavascriptType;
use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;

use std::collections::HashMap;

// Root token class
pub enum Token {
    Tag(TagToken),
    Content(ContentToken),
    InlineJavascript(InlineJavascriptToken),
}

// Represents a single html tag - <opening>, </closing> or <standalone />
pub struct TagToken {
    pub name: String,
    pub attributes: Vec<TagAttribute>, // eg style or id
    pub tag_type: TagType,
}
// Represents (content) inner text of a html node.
pub struct ContentToken {
    pub value: String,
}
// Represents a chunk of javascript found in the markup
pub struct InlineJavascriptToken {
    pub value: String,
    pub javascript_type: JavascriptType,
}

// Represents
// #[derive(Clone)]
// enum JavascriptBracket {
//     Round,
//     Square,
//     Curly,
// }

fn read_markup(markup: &str) -> Vec<Token> {
    let mut remaining = markup.to_string();
    let mut inside_script_tag = false;
    let mut result = vec![];
    while remaining.len() > 0 {
        // Read tag
        if remaining.chars().next().unwrap() == '<' {
            let (tag, chars) = read_html_tag(&remaining);
            remaining.drain(chars..);
            inside_script_tag = tag.name == "<script>" && tag.tag_type == TagType::Start;
            result.push(Token::Tag(tag));
        }
        // Read inline javascript
        else if remaining.chars().next().unwrap() == '~' {
            let inline_js = read_inline_javascript(&remaining);
            remaining.drain(..inline_js.len());
            result.push(Token::InlineJavascript(InlineJavascriptToken {
                value: inline_js,
                javascript_type: find_javascript_type(inline_js),
            }))
        }
        // Read script tag content
        else if inside_script_tag {
            let js = read_javascript(markup);
            remaining.drain(..js.len());
        }
        // Read normal tag content
        else {
            let content = read_tag_content(markup);
            remaining.drain(..content.len());
        }
    }
    return result;
}

fn read_html_tag(markup: &str) -> (TagToken, usize) {
    // Read an open/close/standalone tag. Second return value is tag length

    return (
        TagToken {
            name: "".to_string(),
            attributes: vec![],
            tag_type: TagType::Start,
        },
        5,
    );
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
            value: result,
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
