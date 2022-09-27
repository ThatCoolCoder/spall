use std::fs;
use std::path::Path;

use crate::compilation_settings::*;
use crate::errs;
use crate::logging;
use crate::scoped_css::tokeniser;
use crate::scoped_css::tokeniser::CssToken;

pub fn compile_scoped_css_file(
    file_path: &Path,
    compilation_settings: &CompilationSettings,
) -> Result<String, errs::FileCompilationError> {
    // Compile scoped css directly from a file, determining element name to target based on the file name

    let file_content = fs::read_to_string(file_path).expect(&format!(
        "Failed reading scoped css file: {}",
        file_path.to_string_lossy()
    ));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    compile_scoped_css(&file_content, &element_name, compilation_settings)
}

pub fn compile_scoped_css(
    file_content: &str,
    element_name: &str,
    compilation_settings: &CompilationSettings,
) -> Result<String, errs::FileCompilationError> {
    // Compile scoped css from a string
    // Requires explicit setting of the element name
    // In addition to tweaking the styles, also has the effect of normalizing the style
    // Produced css is not optimal (it contains much whitespace), it may be desirable to run it through a minifier

    logging::log_brief(
        format!("Compiling scoped CSS for element {element_name}").as_str(),
        compilation_settings.log_level,
    );

    // Tokenise
    let tokens = tokeniser::tokenise_css(file_content)
        .or_else(|e| Err(errs::FileCompilationError::CssSyntaxError(e)))?;

    // Write tokens back to a string, making required modifications as we go
    let mut result = "".to_string();
    for token in tokens {
        result += match token {
            CssToken::BlockEnd => "}\n\n".to_string(),
            CssToken::BlockStart => " {\n".to_string(),
            CssToken::Colon => ": ".to_string(),
            CssToken::Comma => ", ".to_string(),
            CssToken::Comment(value) => format!("/* {value} */"),
            CssToken::PropertyName(name) => format!("    {name}"),
            CssToken::PropertyValue(value) => value,
            CssToken::Semicolon => ";\n".to_string(),

            // This is the special one where we mess with the class names
            CssToken::Selector(value) => format!("._sp{element_name} {value}"),
        }
        .as_str();
    }
    Ok(result)
}
