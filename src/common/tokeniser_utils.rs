// Misc small functions that could be useful in tokenising various languages

pub fn get_char_unwrap(data: &str, idx: usize) -> char {
    data.chars().nth(idx).unwrap()
}

pub fn read_string(quote_char: char, escape_char: char, data: &str) -> String {
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

pub fn read_whitespace(data: &str) -> String {
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

pub fn read_until_char(data: &str, char: char) -> (String, bool) {
    // Read a string until a certain character is found.
    // Second return value indicates if the character was found before EOF
    // currently uses a comparitively slow implementation wrapping read_until_any_of

    let (string, found_char) = read_until_any_of(data, &vec![char]);
    (string, found_char.is_some())
}

pub fn read_until_any_of(data: &str, chars: &Vec<char>) -> (String, Option<char>) {
    // Read a string until (but not including) one of chars.
    // If the char is not found then returns the whole string.
    // Second return value is the char that is stopped at. None if went until end

    let mut idx: usize = 0;
    let mut result = "".to_string();
    let mut found_char = None;
    while idx < data.len() {
        let crnt_char = get_char_unwrap(data, idx);
        if chars.contains(&crnt_char) {
            found_char = Some(crnt_char);
            break;
        }
        result.push(crnt_char);
        idx += 1;
    }
    (result, found_char)
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_read_until_character() {
        // todo: write this!!!
    }
}
