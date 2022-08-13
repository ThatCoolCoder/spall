use crate::javascript_type::JavascriptType;
use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;

enum TokeniserState {
    Unknown,
    ReadingTag,
    ReadingTagContent,
    ReadingJavascript,
}

pub enum Token {
    Tag(TagToken),
    Content(ContentToken),
    Javascript(JavascriptToken),
}

pub struct TagToken {
    pub name: String,
    pub attributes: Vec<TagAttribute>, // eg style or id
    pub tag_type: TagType,
}
pub struct ContentToken {
    pub value: String,
}
pub struct JavascriptToken {
    pub value: String,
    pub javascript_type: JavascriptType,
}

pub fn tokenise_element(element: &str) -> Vec<Token> {
    let mut state = TokeniserState::Unknown;
    let mut remaining_chars: Vec<char> = element.chars().collect();

    let mut tokens = vec![];

    while remaining_chars.len() > 0 {
        match &state {
            TokeniserState::Unknown => {
                state = match remaining_chars.first().unwrap() {
                    '<' => TokeniserState::ReadingTag,
                    '~' => TokeniserState::ReadingJavascript,
                    _ => TokeniserState::ReadingTagContent,
                }
            }
            TokeniserState::ReadingTag => {
                let mut tag: String = remaining_chars
                    .clone()
                    .into_iter()
                    .take_while(|c| *c != '>')
                    .collect();
                remaining_chars.drain(..tag.len());
                tag.push(remaining_chars.remove(0)); // add last angle bracket

                // Remove the angle brackets
                let mut tag_content = tag.clone();
                tag_content.pop();
                tag_content.remove(0);

                let tag_type = find_tag_type(&tag_content);
                let content_sections = tag_content.splitn(2, ' ').collect::<Vec<&str>>();
                let tag_name = content_sections[0];
                let raw_tag_attributes = if content_sections.len() >= 2 {
                    content_sections[1]
                } else {
                    ""
                };

                tokens.push(Token::Tag(TagToken {
                    name: tag_name.to_string(),
                    attributes: parse_tag_attributes(raw_tag_attributes),
                    tag_type,
                }));
                state = TokeniserState::Unknown;
            }
            TokeniserState::ReadingTagContent => {
                let content: String = remaining_chars
                    .clone()
                    .iter()
                    .take_while(|c| **c != '<' && **c != '~')
                    .collect();
                remaining_chars.drain(..content.len());
                if content.len() > 0 {
                    tokens.push(Token::Content(ContentToken {
                        value: content.clone(),
                    }));
                }
                state = TokeniserState::Unknown;
            }
            TokeniserState::ReadingJavascript => {
                let content: String = remaining_chars
                    .clone()
                    .iter()
                    .skip(1)
                    .take_while(|c| **c != '~' && **c != '\n')
                    .collect();
                remaining_chars.drain(..content.len() + 2);
                if content.len() > 0 {
                    let javascript_type = find_javascript_type(&content);
                    tokens.push(Token::Javascript(JavascriptToken {
                        value: content.clone(),
                        javascript_type: javascript_type,
                    }));
                }
                state = TokeniserState::Unknown;
            }
        }
    }

    return tokens;
}

fn find_tag_type(tag_content: &str) -> TagType {
    // Tag content should not include angle brackets

    // </tag>
    if tag_content.chars().next().unwrap() == '/' {
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

fn parse_tag_attributes(raw_data: &str) -> Vec<TagAttribute> {
    return raw_data
        .split(" ")
        .filter_map(|x| {
            let sections = x.split("=").collect::<Vec<&str>>();
            match sections.len() {
                2 => Some(TagAttribute {
                    name: sections[0].to_string(),
                    value: sections[1].to_string(),
                }),
                _ => None,
            }
        })
        .collect::<Vec<TagAttribute>>();
}
