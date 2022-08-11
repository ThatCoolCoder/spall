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
    Javascript(JavascriptToken),
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
pub struct JavascriptToken {
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

fn read_html_tag() {}

fn read_tag_content() {}

fn read_javascript(data: &str) -> String {
    // Read some JavaScript until it is ended by a <script> tag.
    // Does not return the script tag

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
    }
    return result;
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
