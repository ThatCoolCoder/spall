// Misc small functions that could be used in multiple compilers

pub fn escape_quotes(data: &str, quote_char: char, escape_char: char) -> String {
    data.replace(escape_char, format!("{escape_char}{escape_char}").as_str())
        .replace(quote_char, format!("{escape_char}{quote_char}").as_str())
}