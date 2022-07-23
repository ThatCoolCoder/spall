// Converts a .spall file into a javascript file

use crate::errs::*;
use crate::{parser, tokeniser};
use std::fs;
use std::path::Path;

const ROOT_ELEMENT_NAME: &str = "Root";
// const HTML_TAGS: Vec<&str> = vec!["html", "head", "body", "h1", "p", "html", "html", "html", "html", "html", "html"];

enum RenderableType {
    PlainMarkup,
    Element,
}

pub fn compile_element_file(file_path: &Path) -> Result<String, CompilationError> {
    // todo: if is not a .spall file: crash

    let file_content = fs::read_to_string(file_path).expect(&format!(
        "Failed reading element file: {}",
        file_path.to_string_lossy()
    ));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    return compile_element(&file_content, &element_name);
}

pub fn compile_element(file_content: &str, element_name: &str) -> Result<String, CompilationError> {
    // Preparation

    if !element_name_valid(element_name) {
        return Err(CompilationError::InvalidElementName {
            name: element_name.to_owned(),
        });
    }

    let compiled_element_name = generate_compiled_element_name(element_name);
    let base_class = if element_name == ROOT_ELEMENT_NAME {
        "SpallRootElement"
    } else {
        "SpallElement"
    };

    // Processing

    let tokens = tokeniser::tokenise_element(file_content);
    let tree = parser::parse_element(&tokens);

    // Building
    let mut renderables = vec![];

    tree.depth_first_map(&mut |node, is_entering| match node.parent {
        // (Ignore root node)
        Some(_) => {
            let renderable = renderable_from_node_visit(node, is_entering);
            match renderable {
                Some(v) => renderables.push(v),
                _ => ()
            }
        }
        None => (),
    });

    let joined_renderables = renderables.join(", ");

    // let renderables = tokens.match()

    let result = format!(
        r#"
        class {compiled_element_name} extends {base_class} {{
            constructor(id, parentId) {{
                super('{element_name}', id, parentId);
            }}

            generateRenderables() {{
                return [{joined_renderables}];
            }}
        }}
    "#
    );

    return Ok(result);
}

fn renderable_from_node_visit(node: &parser::Node, is_entering: bool) -> Option<String> {
    let is_element = node.tag_name.chars().next().unwrap().is_uppercase();

    if is_element {
        if is_entering {
            return Some(format!(
                r#"new SpallElementRenderable("{}", {})"#,
                node.tag_name,
                generate_compiled_element_name(&node.tag_name)
            ));
        }
        else {
            return None;
        }
    } else {
        let markup_string = match is_entering {
            true => format!("<{}>{}", node.tag_name, node.inner_text),
            false => format!("</{}>", node.tag_name),
        };
        return Some(format!(
            "new SpallMarkupRenderable(`{}`)",
            escape_quotes(&markup_string, '`', '\\')
        ));
    }
}

fn generate_compiled_element_name(element_name: &str) -> String {
    return format!("__SpallCompiled{element_name}");
}

fn element_name_valid(element_name: &str) -> bool {
    if element_name.len() == 0 {
        return false;
    }
    if !element_name.chars().next().unwrap().is_alphabetic() {
        return false;
    }
    if element_name.chars().any(|c| !c.is_alphanumeric()) {
        return false;
    }
    return true;
}

fn escape_quotes(data: &str, quote_char: char, escape_char: char) -> String {
    return data
        .replace(escape_char, format!("{escape_char}{escape_char}").as_str())
        .replace(quote_char, format!("{escape_char}{quote_char}").as_str());
}
