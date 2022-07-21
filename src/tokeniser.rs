enum TokeniserState {
    ReadingTag,
    ReadingTagContent,
}

pub enum TokenType {
    Tag { name: String, is_start: bool },
    Content { value: String },
}

pub fn tokenise_element(element: &str) -> Vec<TokenType> {
    let mut state = TokeniserState::ReadingTag;
    let mut remaining_chars: Vec<char> = element.chars().collect();

    let mut tokens = vec!();

    while remaining_chars.len() > 0 {
        match &state {
            TokeniserState::ReadingTag => {
                let mut tag_name: String = remaining_chars.clone().into_iter().skip(1).take_while(|c| *c != '>').collect();
                let is_start = tag_name.chars().next().unwrap() != '/';
                remaining_chars.drain(..tag_name.len() + 2); // (+ 2 for the angle brackets)
                if ! is_start {
                    tag_name.remove(0); // (for the slash)
                }

                tokens.push(TokenType::Tag { name: tag_name, is_start });
                state = TokeniserState::ReadingTagContent;
            },
            TokeniserState::ReadingTagContent => {
                let content: String = remaining_chars.clone().into_iter().take_while(|c| *c != '<').collect();
                remaining_chars.drain(..content.len());
                if content.len() > 0 {
                    tokens.push(TokenType::Content { value: content });
                }
                state = TokeniserState::ReadingTag;
            }
        }
    }

    return tokens;
}