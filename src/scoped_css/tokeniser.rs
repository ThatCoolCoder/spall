// Pretty basic tokeniser for CSS, to allow scoped css
// Does not appreciate complex stuff like styles inside blocks - eg media queries

// Current limitations:
// - Does not read comments
// - Does not work at all

use crate::errs;
use crate::misc::tokeniser_utils;

#[derive(Debug)]
pub enum CssToken {
    Comment(String),
    Selector(String),
    Comma,
    BlockStart,
    BlockEnd,
    PropertyName(String),
    Colon,
    PropertyValue(String),
    Semicolon,
}

pub fn tokenise_css(css: &str) -> Result<Vec<CssToken>, errs::CssSyntaxError> {
    let mut tokens = vec![];
    let mut idx: usize = 0;
    while idx < css.len() {
        let (selector_tokens, chars_read) = read_selectors(&css[idx..])?;
        idx += chars_read;
        tokens.extend(selector_tokens);

        let (property_tokens, chars_read) = read_all_css_properties(&css[idx..])?;
        idx += chars_read;
        tokens.extend(property_tokens);
    }

    Ok(tokens)
}

fn read_selectors(css: &str) -> Result<(Vec<CssToken>, usize), errs::CssSyntaxError> {
    // Read selectors until block open. Returned tokens include the block open

    let mut result = vec![];
    let mut idx: usize = 0;
    while idx < css.len() {
        // Read selector and process found char
        let (crnt_selector, found_char_opt) =
            tokeniser_utils::read_until_any_of(&css[idx..], &vec![',', '{']);
        let found_char = found_char_opt.ok_or(errs::CssSyntaxError::UnexpectedEndOfFile)?;

        // Add selector to tokens
        let cleaned_selector = crnt_selector.trim();
        if cleaned_selector.len() > 0 {
            result.push(CssToken::Selector(cleaned_selector.to_string()));
        }
        idx += crnt_selector.len();

        // if is comma: add token, +1 idx
        if found_char == ',' {
            result.push(CssToken::Comma);
        } else {
            result.push(CssToken::BlockStart);
            break;
        }
    }
    Ok((result, idx))
}

fn read_all_css_properties(css: &str) -> Result<(Vec<CssToken>, usize), errs::CssSyntaxError> {
    // Read css properties until a block close
    // Returned tokens include the bock close

    let mut result = vec![];
    let mut idx: usize = 0;
    while idx < css.len() {
        idx += tokeniser_utils::read_whitespace(&css[idx..]).len();
        if tokeniser_utils::get_char_unwrap(css, idx) == '}' {
            result.push(CssToken::BlockEnd);
            idx += 1;
            break;
        }

        let (new_tokens, chars_read) = read_css_property(&css[idx..])?;
        idx += chars_read;
        result.extend(new_tokens);
    }
    Ok((result, idx))
}

fn read_css_property(css: &str) -> Result<(Vec<CssToken>, usize), errs::CssSyntaxError> {
    let mut chars_read: usize = 0;
    let mut result = vec![];

    let (property_name, found_colon) = tokeniser_utils::read_until_char(css, ':');
    if !found_colon {
        Err(errs::CssSyntaxError::UnexpectedEndOfFile)?
    }

    chars_read += property_name.len() + 1; // +1 to account for the colon token that we add just below
    result.push(CssToken::PropertyName(property_name.trim().to_string()));
    result.push(CssToken::Colon);

    let (property_value, found_semicolon) =
        tokeniser_utils::read_until_char(&css[..chars_read], ';');
    if !found_semicolon {
        Err(errs::CssSyntaxError::UnexpectedEndOfFile)?
    }

    chars_read += property_value.len();
    result.push(CssToken::PropertyValue(property_value.trim().to_string()));
    result.push(CssToken::Semicolon);

    Ok((result, chars_read))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_selectors() {
        let selectors = read_selectors(".main, .big {").unwrap();
        println!("{selectors:?}");
    }
}
