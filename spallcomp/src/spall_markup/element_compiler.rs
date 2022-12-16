// Converts a .spall file into a javascript file

use std::fs;
use std::path::Path;

use crate::compilation_settings::*;
use crate::errs;
use crate::logging;
use super::ElementType;
use super::tag_type::TagType;
use super::element_metadata::{determine_element_metadata};
use super::{parser, tokeniser, text_generation};

pub struct CompiledElement {
    pub content: String,
    pub element_name: String,
    pub compiled_element_name: String,
}

pub fn compile_element_file(
    file_path: &Path,
    compilation_settings: &CompilationSettings,
    element_type: ElementType,
    element_id: i32,
) -> Result<CompiledElement, errs::FileCompilationError> {
    // todo: if is not a .spall file: crash?

    let file_content = fs::read_to_string(file_path).expect(&format!(
        "Failed reading element file: {}",
        file_path.to_string_lossy()
    ));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    compile_element(
        &file_content,
        &element_name,
        compilation_settings,
        element_type,
        element_id,
    )
}

// How the general flow of compilation works:
// First, we do a bit of set up like figuring out the element name and checking it.
// Then we tokenise the element and then we turn the element into a node tree.
// We try to extract the class body (main script) from the element.
// We turn the tree into a group of compile chunks, which are either javascript or renderables.
// We simplify the chunks since otherwise it's stupidly inneficient.
// We then compile the chunks into a single string of javascript - javascript chunks are pasted directly in,
// while renderable chunks are converted into javascript code to generate renderables in the runtime.

pub fn compile_element(
    file_content: &str,
    element_name: &str,
    compilation_settings: &CompilationSettings,
    element_type: ElementType,
    element_id: i32,
) -> Result<CompiledElement, errs::FileCompilationError> {
    logging::log_brief(
        format!("Compiling element {}", element_name).as_str(),
        compilation_settings.log_level,
    );

    let metadata = determine_element_metadata(element_name, element_type)?;

    logging::log_per_step("Tokenising", compilation_settings.log_level);
    let tokens = tokeniser::read_element(file_content);
    if compilation_settings.debug_tokens {
        debug_tokens(&tokens);
    }
    check_token_syntax(&tokens)
        .or_else(|e| Err(errs::FileCompilationError::MarkupSyntaxError(e)))?;
    logging::log_per_step("Parsing", compilation_settings.log_level);
    let tree = parser::parse_element(&tokens)
        .or_else(|e| Err(errs::FileCompilationError::MarkupSyntaxError(e)))?;

    let compiled_element_text = text_generation::compile_tree(compilation_settings, &metadata, &tree)?;

    Ok(CompiledElement {
        content: compiled_element_text,
        element_name: element_name.to_string(),
        compiled_element_name: metadata.compiled_element_name.to_string(),
    })
}

fn check_token_syntax(tokens: &Vec<tokeniser::Token>) -> Result<(), errs::MarkupSyntaxError> {
    for token in tokens {
        if let tokeniser::Token::Tag(tag) = token {
            if tag.tag_type == TagType::End && tag.attributes.len() > 0 {
                return Err(errs::MarkupSyntaxError::AttributesOnCloseTag {
                    tag_name: tag.name.clone(),
                });
            }
        }
    }
    Ok(())
}

fn debug_tokens(tokens: &Vec<tokeniser::Token>) {
    let data = tokens
        .iter()
        .map(|token| match token {
            tokeniser::Token::Tag(inner_data) => inner_data.to_string(),
            tokeniser::Token::Content(inner_data) => inner_data.to_string(),
            tokeniser::Token::InlineJavascript(inner_data) => inner_data.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ");
    println!("{data}");
}