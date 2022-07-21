// Converts a .spall file into a javascript file 

use std::path::Path;
use std::fs;
use crate::errs::*;
use crate::tokeniser;

const ROOT_ELEMENT_NAME: &str = "Root";

pub fn compile_element_file(file_path: &Path) -> Result<String, CompilationError> {
    // if is not a .spall file: crash

    let file_content = fs::read_to_string(file_path)
        .expect(&format!("Failed reading element file: {}", file_path.to_string_lossy()));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    return compile_element(&file_content, &element_name);
}

pub fn compile_element(file_content: &str, element_name: &str) -> Result<String, CompilationError> {
    if ! element_name_valid(element_name) {
        return Err(CompilationError::InvalidElementName { name: element_name.to_owned() } );
    }

    let uncompiled = file_content.to_owned();
    let compiled_element_name = generate_compiled_element_name(element_name);
    let base_class = if element_name == ROOT_ELEMENT_NAME { "SpallRootElement" } else { "SpallElement" };

    let tokens = tokeniser::tokenise_element(file_content);
    for token in tokens {
        match token {
            tokeniser::Token::Tag {name, is_start} => {
                println!("Token tag, name is {name} and is start is {is_start}");
            }
            tokeniser::Token::Content {value} => {
                println!("Content tag, value is {value}");
            }
        }
    }

    let escaped = escape_quotes(&uncompiled, '`', '\\');

    // let renderables = tokens.match()

    let result = format!(r#"
        class {compiled_element_name} extends {base_class} {{
            constructor() {{
                super('{element_name}');
            }}

            generateRenderables() {{
                return [`{escaped}`];
            }}
        }}
    "#);

    return Ok(result);
}

fn generate_compiled_element_name(element_name: &str) -> String {
    return format!("__SpallCompiled{element_name}");
}

fn element_name_valid(element_name: &str) -> bool {
    if element_name.len() == 0 {
        return false;
    }
    if ! element_name.chars().next().unwrap().is_alphabetic() {
        return false;
    }
    if element_name.chars().any(|c| ! c.is_alphanumeric()) {
        return false;
    }
    return true;
}

fn escape_quotes(data: &str, quote_char: char, escape_char: char) -> String {
    return data.replace(escape_char, format!("{escape_char}{escape_char}").as_str())
        .replace(quote_char, format!("{escape_char}{quote_char}").as_str());
}