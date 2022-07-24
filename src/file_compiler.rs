// Converts a .spall file into a javascript file

use crate::errs::*;
use crate::{parser, tokeniser};
use std::fs;
use std::path::Path;

const ROOT_ELEMENT_NAME: &str = "Root";
// const HTML_TAGS: Vec<&str> = vec!["html", "head", "body", "h1", "p", "html", "html", "html", "html", "html", "html"];

enum Renderable {
    Markup(String),
    Element {
        tag_name: String,
        compiled_element_name: String,
        path: String,
    },
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

    // Reading/ parsing

    let tokens = tokeniser::tokenise_element(file_content);
    let tree = parser::parse_element(&tokens);

    // Building/writing

    let mut renderables = renderables_from_tree(&tree);
    renderables = simplify_renderables(&renderables);
    let stringified_renderables = renderables_to_string(&renderables);

    let result = format!(
        r#"
        class {compiled_element_name} extends {base_class} {{
            constructor(id, parentId) {{
                super('{element_name}', id, parentId);
            }}

            generateRenderables() {{
                return [{stringified_renderables}];
            }}
        }}
    "#
    );

    return Ok(result);
}

fn renderables_from_tree(tree: &parser::Tree) -> Vec<Renderable> {
    let mut renderables = vec![];
    // I don't know why the code for tracking the path stack works, but it does
    let mut path_stack = vec![0];

    tree.depth_first_map(&mut |node, is_entering| match node.parent {
        // (Ignore root node)
        Some(_) => {
            let path = path_stack
                .iter()
                .map(|x: &i32| x.to_string())
                .collect::<Vec<String>>()
                .join("/");
            if is_entering {
                path_stack.push(0);
            } else {
                path_stack.pop();
                if path_stack.len() > 0 {
                    let idx = path_stack.len() - 1;
                    path_stack[idx] += 1;
                }
            }
            let renderable = renderable_from_node_visit(node, is_entering, &path);
            match renderable {
                Some(v) => renderables.push(v),
                _ => (),
            }
        }
        None => (),
    });
    return renderables;
}

fn renderable_from_node_visit(
    node: &parser::Node,
    is_entering: bool,
    path: &str,
) -> Option<Renderable> {
    let is_element = node.tag_name.chars().next().unwrap().is_uppercase();

    if is_element {
        if is_entering {
            return Some(Renderable::Element {
                tag_name: node.tag_name.clone(),
                compiled_element_name: generate_compiled_element_name(&node.tag_name),
                path: path.to_string(),
            });
        } else {
            return None;
        }
    } else {
        let markup_string = match (node.is_standalone, is_entering) {
            (true, true) => format!("<{} />", node.tag_name),
            (true, false) => return None,
            (false, true) => format!("<{}>{}", node.tag_name, node.inner_text),
            (false, false) => format!("</{}>", node.tag_name),
        };
        return Some(Renderable::Markup(markup_string));
    }
}

fn simplify_renderables(renderables: &Vec<Renderable>) -> Vec<Renderable> {
    return join_successive_markup_renderables(renderables);
}

fn join_successive_markup_renderables(renderables: &Vec<Renderable>) -> Vec<Renderable> {
    let mut new_renderables = vec![];
    let mut crnt_markup_string = "".to_string();
    for renderable in renderables {
        match renderable {
            Renderable::Markup(markup_string) => {
                crnt_markup_string += markup_string;
            }
            // Surely there's got to be a better way than deconstructing and reconstructing the whole enum if we don't want to modify it
            Renderable::Element {
                tag_name,
                compiled_element_name,
                path,
            } => {
                new_renderables.push(Renderable::Markup(crnt_markup_string));
                crnt_markup_string = "".to_string();
                new_renderables.push(Renderable::Element {
                    tag_name: tag_name.clone(),
                    compiled_element_name: compiled_element_name.clone(),
                    path: path.clone(),
                });
            }
        }
    }
    if crnt_markup_string.len() > 0 {
        new_renderables.push(Renderable::Markup(crnt_markup_string));
    }

    return new_renderables;
}

fn renderables_to_string(renderables: &Vec<Renderable>) -> String {
    let mut stringified_renderables = vec![];
    for renderable in renderables {
        let string_val = match renderable {
            Renderable::Markup(value) => format!(
                "new SpallMarkupRenderable(`{}`)",
                escape_quotes(&value, '`', '\\')
            ),
            Renderable::Element {
                tag_name,
                compiled_element_name,
                path,
            } => format!(
                r#"new SpallElementRenderable("{tag_name}", {compiled_element_name}, "{path}")"#
            ),
        };
        stringified_renderables.push(string_val);
    }

    return stringified_renderables.join(", ");
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
