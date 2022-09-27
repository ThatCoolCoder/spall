// Pretty basic tokeniser for CSS, to allow scoped css
// Does not appreciate complex stuff like styles inside blocks - eg media queries

// Current limitations:
// - Does not read comments

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
    // Convert a css stream into a vec of tokens

    let mut tokens = vec![];
    let mut idx: usize = 0;
    while idx < css.len() {
        // Read selectors until we get to the start of the properties
        let (selector_tokens, chars_read) = read_selectors(&css[idx..])?;
        idx += chars_read;
        tokens.extend(selector_tokens);

        // Read properties until we get to the end of those
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
            idx += 1;
        } else {
            result.push(CssToken::BlockStart);
            idx += 1;
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
        // Skip whitespace so we can see if we're at block close
        idx += tokeniser_utils::read_whitespace(&css[idx..]).len();
        if tokeniser_utils::get_char_unwrap(css, idx) == '}' {
            result.push(CssToken::BlockEnd);
            idx += 1;
            break;
        }

        // If we're not then read a property.
        let (new_tokens, chars_read) = read_css_property(&css[idx..])?;
        idx += chars_read;
        result.extend(new_tokens);
    }
    Ok((result, idx))
}

fn read_css_property(css: &str) -> Result<(Vec<CssToken>, usize), errs::CssSyntaxError> {
    let mut chars_read: usize = 0;
    let mut result = vec![];

    // Read up until the colon separating name from value
    let (property_name, found_colon) = tokeniser_utils::read_until_char(css, ':');
    if !found_colon {
        Err(errs::CssSyntaxError::UnexpectedEndOfFile)?
    }
    chars_read += property_name.len() + 1; // +1 to account for the colon token that we add just below
    result.push(CssToken::PropertyName(property_name.trim().to_string()));
    result.push(CssToken::Colon);

    // Read up until the semicolon at end of line
    let (property_value, found_semicolon) =
        tokeniser_utils::read_until_char(&css[chars_read..], ';');
    if !found_semicolon {
        Err(errs::CssSyntaxError::UnexpectedEndOfFile)?
    }

    chars_read += property_value.len() + 1; // +1 to account for semicolon token
    result.push(CssToken::PropertyValue(property_value.trim().to_string()));
    result.push(CssToken::Semicolon);

    Ok((result, chars_read))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_selectors() {
        let (tokens, len) = read_selectors(".main, .big {").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(len, 13);

        let (tokens, _) = read_selectors("{").unwrap();
        assert_eq!(tokens.len(), 1);

        let result = read_selectors(".main, .big");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_read_all_css_properties() {
        let (tokens, len) = read_all_css_properties("}aaaa").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(len, 1);

        let (tokens, len) = read_all_css_properties("color: red; }aaa").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(len, 13);

        let (tokens, len) = read_all_css_properties("color: red; background: blue; }aaa").unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(len, 31);
    }

    #[test]
    fn test_read_css_property() {
        let (tokens, len) = read_css_property("color: red; ").unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(len, 11);
    }
}
