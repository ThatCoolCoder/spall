use crate::javascript_type::JavascriptType;
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
                let tag_name = find_tag_name(&tag_content);

                tokens.push(Token::Tag(TagToken {
                    name: tag_name,
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
                state = TokeniserState::Unknown
            }
            TokeniserState::ReadingJavascript => {
                let content: String = remaining_chars
                    .clone()
                    .iter()
                    .take_while(|c| **c != '~')
                    .collect();
                remaining_chars.drain(..content.len());
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

fn find_tag_name(tag_content: &str) -> String {
    // Tag content should not include angle brackets

    return tag_content.replace('/', "").replace(' ', "");
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
