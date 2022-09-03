// WIP new tokeniser that actually parses things well.
// The previous implementation used iterators and lambdas but I've decided to use plain for-loops this time,
// as the iterators became too complex when implementing complex patterns

use std::collections::HashMap;
use std::fmt;

use derive_more::Display;

use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;

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
impl fmt::Display for TagToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.attributes.len() > 0 {
            let attributes_str = self
                .attributes
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .join(" ");
            write!(
                f,
                "[{} {} tag with {}]",
                self.tag_type, self.name, attributes_str
            )
        } else {
            write!(f, "[{} {} tag]", self.tag_type, self.name)
        }
    }
}
// Represents (content) inner text of a html node.
#[derive(Display)]
#[display(fmt = "[Content: {value}]")]
pub struct ContentToken {
    pub value: String,
}
// Represents a chunk of javascript found in the markup
#[derive(Display)]
#[display(fmt = "[Inline javascript: {value}]")]
pub struct InlineJavascriptToken {
    pub value: String,
}

pub fn read_element(markup: &str) -> Vec<Token> {
    let mut remaining = markup.to_string();
    let mut inside_script_tag = false;
    let mut result = vec![];
    while remaining.len() > 0 {
        // Read tag
        if remaining.chars().next().unwrap() == '<' {
            let (tag, chars) = read_html_tag(&remaining);
            remaining.drain(..chars);
            inside_script_tag = tag.name == "<script>" && tag.tag_type == TagType::Start;
            result.push(Token::Tag(tag));
        }
        // Read inline javascript
        else if remaining.chars().next().unwrap() == '~' {
            let (inline_js, size) = read_inline_javascript(&remaining);
            remaining.drain(..size);
            result.push(Token::InlineJavascript(inline_js));
        }
        // Read script tag content
        else if inside_script_tag {
            let js = read_javascript(&remaining);
            remaining.drain(..js.len());
            result.push(Token::Content(ContentToken { value: js }));
        }
        // Read normal tag content
        else {
            let content = read_tag_content(&remaining);
            remaining.drain(..content.len());
            if !content.trim().is_empty() {
                result.push(Token::Content(ContentToken { value: content }));
            }
        }
    }
    result
}

fn read_html_tag(markup: &str) -> (TagToken, usize) {
    // Read an open/close/standalone tag. Second return value is tag length

    let mut tag_name = "".to_string();
    let mut idx = 1; // start at 1 to skip the opening "<"
    let mut tag_type = TagType::Start;
    let mut found_end_tag = false;

    // Read tag name
    while idx < markup.len() {
        let char = get_char_unwrap(markup, idx);
        match char {
            ' ' => {
                idx += 1;
                break;
            }
            '>' => {
                idx += 1;
                found_end_tag = true;
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
        idx += 1;
    }

    tag_name = tag_name.trim().to_string();

    let mut tag_attributes = vec![];
    if !found_end_tag {
        let (_tag_attributes, len) = read_tag_attributes(&markup[idx..]);
        tag_attributes = _tag_attributes;
        idx += len;

        while idx < markup.len() {
            let char = get_char_unwrap(&markup, idx);
            idx += 1;
            match char {
                '/' => {
                    tag_type = TagType::Standalone;
                }
                '>' => {
                    break;
                }
                _ => (),
            }
        }
    }
    (
        TagToken {
            name: tag_name,
            attributes: tag_attributes,
            tag_type: tag_type,
        },
        idx,
    )
}

fn read_tag_attributes(data: &str) -> (Vec<TagAttribute>, usize) {
    // Read tag attributes until the end of a html tag.

    let mut idx: usize = 0;
    let mut tag_attributes = vec![];
    while idx < data.len() {
        // skip forward if there are any spaces
        idx += read_whitespace(&data[idx..]).len();

        // check if we are at end of tag
        let char = get_char_unwrap(data, idx);
        if char == '/' || char == '>' {
            break;
        }

        // read a tag attribute using the other func
        let (attribute, len) = read_tag_attribute(&data[idx..]);
        tag_attributes.push(attribute);
        idx += len;
    }
    (tag_attributes, idx)
}

fn read_tag_attribute(data: &str) -> (TagAttribute, usize) {
    // Read a tag attribute up until the string value finishes

    let mut idx: usize = 0;
    let mut attribute_name = "".to_string();

    let is_dynamic = data.chars().next().unwrap() == '!'; // todo: throw EOF error instead of unwrap
    if is_dynamic {
        idx += 1;
    }

    while idx < data.len() {
        let char = get_char_unwrap(&data, idx);
        if char == ' ' || char == '=' {
            break;
        } else {
            attribute_name.push(char);
        }
        idx += 1;
    }

    idx += read_whitespace(&data[idx..]).len();
    // if idx > len: err(you messed up)
    idx += 1; // jump over equals sign
              // if idx > len: err(you messed up)
    idx += read_whitespace(&data[idx..]).len();

    let mut attribute_value = read_string(get_char_unwrap(data, idx), '\\', &data[idx..]);
    idx += attribute_value.len();
    attribute_value.pop();
    attribute_value.remove(0);
    (
        TagAttribute {
            name: attribute_name,
            value: attribute_value,
            is_dynamic: is_dynamic,
        },
        idx,
    )
}

fn get_char_unwrap(data: &str, idx: usize) -> char {
    data.chars().nth(idx).unwrap()
}

fn read_tag_content(markup: &str) -> String {
    // Read the content (inner text) of a tag.
    let mut result = "".to_string();
    for char in markup.chars() {
        if char == '<' || char == '~' {
            break;
        }
        result.push(char);
    }
    result
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
    let mut char_idx = 0;
    while char_idx < data.len() {
        let char = data.chars().nth(char_idx).unwrap();
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
        if bracket_stack.len() == 0 && data[char_idx..].starts_with("</script>") {
            break;
        }
    }
    result
}

fn read_inline_javascript(markup: &str) -> (InlineJavascriptToken, usize) {
    // Read inline javascript.
    // Expects markup to begin with a tilde (inline start char)

    let mut result = "".to_string();
    let mut reached_end = false;
    for char in markup.chars().skip(1) {
        if char == '~' || char == '\n' {
            reached_end = true;
            break;
        }
        result.push(char);
    }
    let mut length = result.len() + 1; // +1 to account for start tilde
    if reached_end {
        length += 1; // +1 to account for \n or end tilde
    }
    (
        InlineJavascriptToken {
            value: result.clone(),
        },
        length,
    )
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

        if char == escape_char && !last_char_is_escape {
            last_char_is_escape = true;
        } else {
            last_char_is_escape = false;
        }
    }
    result
}

fn read_whitespace(data: &str) -> String {
    // Read whitespace until another character occurs.
    // Currently counts spaces, tabs and newlines as whitespace

    let mut idx: usize = 0;
    let mut result = "".to_string();
    while idx < data.len() {
        let char = get_char_unwrap(data, idx);
        match char {
            ' ' | '\t' | '\n' | '\r' => {
                result.push(char);
            }
            _ => {
                break;
            }
        }
        idx += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_html_tag() {
        let mut data = read_html_tag("<h1>");
        assert_eq!(data.0.name, "h1");
        assert_eq!(data.0.tag_type, TagType::Start);
        assert_eq!(data.1, 4);

        data = read_html_tag("</paragraph>this bit won't be there");
        assert_eq!(data.0.name, "paragraph");
        assert_eq!(data.0.tag_type, TagType::End);
        assert_eq!(data.1, 12);

        data = read_html_tag("<input />");
        assert_eq!(data.0.name, "input");
        assert_eq!(data.0.tag_type, TagType::Standalone);
        assert_eq!(data.1, 9);

        data = read_html_tag("<input style='red' />");
        assert_eq!(data.0.name, "input");
        assert_eq!(data.0.tag_type, TagType::Standalone);
        assert_eq!(data.1, 21);

        data = read_html_tag("<input/>");
        assert_eq!(data.0.name, "input");
        assert_eq!(data.0.tag_type, TagType::Standalone);
        assert_eq!(data.1, 8);
    }

    #[test]
    fn test_read_tag_attribute() {
        let mut data = read_tag_attribute("style='color: blue'");
        assert_eq!(data.0.name, "style");
        assert_eq!(data.0.value, "color: blue");
        assert_eq!(data.1, 19);

        data = read_tag_attribute("style='color: blue'  ");
        assert_eq!(data.0.name, "style");
        assert_eq!(data.0.value, "color: blue");
        assert_eq!(data.1, 19);

        data = read_tag_attribute(r#"style="color: blue\""  "#);
        assert_eq!(data.0.name, "style");
        assert_eq!(data.0.value, r#"color: blue\""#);
        assert_eq!(data.1, 21);
    }

    #[test]
    fn test_read_tag_content() {
        assert_eq!(read_tag_content("Hello world</h1>"), "Hello world");
    }

    #[test]
    fn test_read_javascript() {
        assert_eq!(read_javascript("var x = 5;</script>"), "var x = 5;");
        assert_eq!(
            read_javascript("var x = '</script>';</script>"),
            "var x = '</script>';"
        );
    }

    #[test]
    fn test_read_inline_javascript() {
        let mut data = read_inline_javascript("~if (x == 5) {\n");
        assert_eq!(data.0.value, "if (x == 5) {");
        assert_eq!(data.1, 15);

        data = read_inline_javascript("~}~");
        assert_eq!(data.0.value, "}");
        assert_eq!(data.1, 3);

        data = read_inline_javascript("~var x = 5~");
        assert_eq!(data.0.value, "var x = 5");
        assert_eq!(data.1, 11);

        data = read_inline_javascript("~var x = 5\n");
        assert_eq!(data.0.value, "var x = 5");
        assert_eq!(data.1, 11);
    }

    #[test]
    fn test_read_string() {
        assert_eq!(
            read_string('\'', '\\', "'this in quotes' end"),
            "'this in quotes'"
        );
        assert_eq!(
            read_string('\'', '\\', r#"'Hello world, don\'t \\' end"#),
            r#"'Hello world, don\'t \\'"#
        );
    }

    #[test]
    fn test_read_whitespace() {
        assert_eq!(read_whitespace("      hello"), "      ");
        assert_eq!(read_whitespace("      \t\n  hello"), "      \t\n  ");
    }
}
