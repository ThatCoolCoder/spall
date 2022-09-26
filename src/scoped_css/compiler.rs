use std::fs;
use std::path::Path;

use crate::errs;
use crate::scoped_css::tokeniser;
use crate::scoped_css::tokeniser::CssToken;

pub fn compile_scoped_css_file(file_path: &Path) -> Result<String, errs::FileCompilationError> {
    let file_content = fs::read_to_string(file_path).expect(&format!(
        "Failed reading scoped css file: {}",
        file_path.to_string_lossy()
    ));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    compile_scoped_css(&file_content, &element_name)
}

pub fn compile_scoped_css(
    file_content: &str,
    element_name: &str,
) -> Result<String, errs::FileCompilationError> {
    let tokens = tokeniser::tokenise_css(file_content)
        .or_else(|e| Err(errs::FileCompilationError::CssSyntaxError(e)))?;
    let mut result = "".to_string();
    for token in tokens {
        result += match token {
            CssToken::BlockEnd => "\n}\n".to_string(),
            CssToken::BlockStart => "{\n".to_string(),
            CssToken::Colon => ": ".to_string(),
            CssToken::Comma => ", ".to_string(),
            CssToken::Comment(value) => format!("/* {value} */"),
            CssToken::PropertyName(name) => name,
            CssToken::PropertyValue(value) => value,
            CssToken::Semicolon => ";\n".to_string(),

            // This is the special one where we mess with the class names
            CssToken::Selector(value) => format!("{element_name} {value}"),
        }
        .as_str();
    }
    Ok(result)
}
