use crate::tag_type::TagType;

enum TokeniserState {
    ReadingTag,
    ReadingTagContent,
}

pub enum Token {
    Tag { name: String, tag_type: TagType },
    Content { value: String },
}

pub fn tokenise_element(element: &str) -> Vec<Token> {
    let mut state = TokeniserState::ReadingTag;
    let mut remaining_chars: Vec<char> = element.chars().collect();

    let mut tokens = vec![];

    while remaining_chars.len() > 0 {
        match &state {
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


                tokens.push(Token::Tag {
                    name: tag_name,
                    tag_type,
                });
                state = TokeniserState::ReadingTagContent;
            }
            TokeniserState::ReadingTagContent => {
                let content: String = remaining_chars
                    .clone()
                    .into_iter()
                    .take_while(|c| *c != '<')
                    .collect();
                remaining_chars.drain(..content.len());
                if content.len() > 0 {
                    tokens.push(Token::Content { value: content });
                }
                state = TokeniserState::ReadingTag;
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
