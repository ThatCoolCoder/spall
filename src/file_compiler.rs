// Converts a .spall file into a javascript file

use std::fs;
use std::path::Path;

use crate::compilation_settings::*;
use crate::errs;
use crate::logging;
use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;
use crate::{parser, tokeniser};

const ROOT_ELEMENT_NAME: &str = "Root";
// Elements that aren't put into the final markup
const IGNORED_ELEMENT_NAMES: [&'static str; 3] = ["title", "pageroute", "script"];

#[derive(Clone, PartialEq)]
pub enum ElementType {
    Basic,
    Page,
}

enum CompileChunk {
    // Chunk of stuff that we need to compile
    Javascript(String),
    Renderable(Vec<Renderable>),
}

#[derive(Clone)]
enum Renderable {
    // Thing that can be rendered by the runtime.
    Markup(String),
    Element {
        tag_name: String,
        compiled_element_name: String,
        path: String,
        parameters: Vec<ElementParameter>,
    },
}

#[derive(Clone)]
struct ElementParameter {
    name: String,
    value: String,
    is_dynamic: bool, // whether it is a plain text value or is executed at render-time.
                      // Dynamic parameters are those which have a ! at the start in the initial HTML,
                      // although when they are put into the structure the ! is stripped
}

pub fn compile_element_file(
    file_path: &Path,
    compilation_settings: &CompilationSettings,
    element_type: ElementType,
) -> Result<String, errs::FileCompilationError> {
    // todo: if is not a .spall file: crash

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
) -> Result<String, errs::FileCompilationError> {
    // Preparation

    logging::log_brief(
        format!("Compiling element {}", element_name).as_str(),
        compilation_settings.log_level,
    );

    if !element_name_valid(element_name) {
        return Err(errs::FileCompilationError::InvalidElementName {
            name: element_name.to_owned(),
        });
    }

    let compiled_element_name = generate_compiled_element_name(element_name);
    let base_class = if element_type == ElementType::Basic {
        if element_name == ROOT_ELEMENT_NAME {
            "SpallRootElement"
        } else {
            "SpallElement"
        }
    } else {
        "SpallPage"
    };

    // Reading/ parsing

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

    // Building/writing

    logging::log_per_step("Actually compiling", compilation_settings.log_level);
    let class_body = find_class_body(&tree).unwrap_or("".to_string());
    let mut chunks = compile_chunks_from_tree(&tree);
    chunks = concat_successive_compile_chunks(&chunks);
    let compiled_render_func = compile_chunks(&chunks);

    let constructor = match element_type {
        ElementType::Basic => {
            format!("super('{element_name}', id, parentId, rendererInstance, path);")
        }
        ElementType::Page => {
            let mut page_title = "".to_string();
            // add code to register as page
            tree.depth_first_map(&mut |node, _is_entering| {
                if let parser::NodeData::Markup(inner_data) = &node.data {
                    if inner_data.tag_name == "title" {
                        page_title = inner_data.inner_text.clone();
                    }
                };
            });
            format!("super('{page_title}', '{element_name}', id, parentId, rendererInstance, path)")
        }
    };

    let mut result = format!(
        r#"
        class {compiled_element_name} extends {base_class} {{
            constructor(id, parentId, rendererInstance, path) {{
                {constructor}
            }}

            compiledGenerateRenderables() {{
                {compiled_render_func}
            }}

            {class_body}
        }}
    "#
    );

    if element_type == ElementType::Page {
        // add code to register as page

        let mut element_route = "".to_string();
        // add code to register as page
        tree.depth_first_map(&mut |node, _is_entering| {
            if let parser::NodeData::Markup(inner_data) = &node.data {
                if inner_data.tag_name == "pageroute" {
                    element_route = inner_data.inner_text.clone();
                }
            };
        });

        result += &format!(
            "SpallRouter.routeToPageClass['{element_route}'] = {compiled_element_name};\n"
        );
    }

    Ok(result)
}

fn generate_compiled_element_name(element_name: &str) -> String {
    format!("__SpallCompiled{element_name}")
}

fn element_name_valid(element_name: &str) -> bool {
    if element_name.len() == 0 {
        false
    } else if !element_name.chars().next().unwrap().is_alphabetic() {
        false
    } else if element_name.chars().any(|c| !c.is_alphanumeric()) {
        false
    } else {
        true
    }
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

fn escape_quotes(data: &str, quote_char: char, escape_char: char) -> String {
    data.replace(escape_char, format!("{escape_char}{escape_char}").as_str())
        .replace(quote_char, format!("{escape_char}{quote_char}").as_str())
}

fn find_class_body(tree: &parser::Tree) -> Option<String> {
    let mut result = None;
    tree.depth_first_map(&mut |node, _is_entering| {
        if let parser::NodeData::Markup(inner_data) = &node.data {
            if inner_data.tag_name == "script" {
                result = Some(inner_data.inner_text.clone());
            }
        }
    });
    result
}

fn compile_chunks_from_tree(tree: &parser::Tree) -> Vec<CompileChunk> {
    let mut chunks = vec![];
    // I don't know why the code for tracking the path stack works, but it does
    let mut path_stack = vec![0];

    tree.depth_first_map(&mut |node, is_entering| {
        // (Ignore root node)
        if node.parent.is_some() {
            // Keep track of path stack
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

            // generate a compile chunk
            match &node.data {
                parser::NodeData::Markup(inner_data) => {
                    let renderable = renderable_from_node_visit(inner_data, is_entering, &path);
                    match renderable {
                        Some(v) => chunks.push(CompileChunk::Renderable(vec![v])),
                        _ => (),
                    }
                }
                parser::NodeData::InlineJavascript(inner_data) => {
                    if is_entering {
                        chunks.push(CompileChunk::Javascript(inner_data.value.clone()));
                    }
                }
            }
        }
    });
    chunks
}

fn renderable_from_node_visit(
    node_data: &parser::NodeMarkupData,
    is_entering: bool,
    path: &str,
) -> Option<Renderable> {
    if IGNORED_ELEMENT_NAMES.contains(&node_data.tag_name.as_str()) {
        return None;
    }

    let is_element = node_data.tag_name.chars().next().unwrap().is_uppercase();

    if is_element {
        if is_entering {
            Some(Renderable::Element {
                tag_name: node_data.tag_name.clone(),
                compiled_element_name: generate_compiled_element_name(&node_data.tag_name),
                path: path.to_string(),
                parameters: node_data
                    .tag_attributes
                    .iter()
                    .map(|attr| ElementParameter {
                        name: attr.name.clone(),
                        value: attr.value.clone(),
                        is_dynamic: attr.is_dynamic,
                    })
                    .collect(),
            })
        } else {
            None
        }
    } else {
        let tag_attributes = compile_tag_attributes(&node_data.tag_attributes, path);
        let markup_string = match (node_data.is_standalone, is_entering) {
            (true, true) => format!("<{} {}/>", node_data.tag_name, tag_attributes),
            (true, false) => return None,
            (false, true) => format!(
                "<{} {}>{}",
                node_data.tag_name, tag_attributes, node_data.inner_text
            ),
            (false, false) => format!("</{}>", node_data.tag_name),
        };
        Some(Renderable::Markup(markup_string))
    }
}

fn compile_tag_attributes(tag_attributes: &Vec<TagAttribute>, _tag_path: &str) -> String {
    tag_attributes
        .iter()
        .map(|x| {
            // for this.x() callbacks, get context for the "this" by lookups through the renderer
            if x.is_dynamic && x.value.starts_with("this.") {
                let this_removed = x.value.replacen("this.", "", 1);
                // take advantage of the way that strings are inserted into js to inject some stuff from runtime into the html
                format!(
                    "{}=\"SpallRenderer.instance.getElementById(${{this.id}}).{}\"",
                    x.name, this_removed
                )
            } else {
                format!("{x}")
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn concat_successive_compile_chunks(chunks: &Vec<CompileChunk>) -> Vec<CompileChunk> {
    // Simplifies compile chunks by concatenating values of ones of same type.

    let mut crnt_renderable_values = vec![];
    let mut crnt_javascript_value = "".to_string();
    let mut result = vec![];

    for chunk in chunks {
        match chunk {
            CompileChunk::Renderable(ref renderables) => {
                if crnt_javascript_value != "" {
                    result.push(CompileChunk::Javascript(crnt_javascript_value));
                    crnt_javascript_value = "".to_string();
                }
                crnt_renderable_values.append(&mut renderables.clone());
            }
            CompileChunk::Javascript(javascript) => {
                if crnt_renderable_values.len() > 0 {
                    result.push(CompileChunk::Renderable(crnt_renderable_values.clone()));
                    crnt_renderable_values = vec![];
                }
                crnt_javascript_value += &javascript;
            }
        }
    }

    if crnt_javascript_value != "" {
        result.push(CompileChunk::Javascript(crnt_javascript_value));
    }
    if crnt_renderable_values.len() > 0 {
        result.push(CompileChunk::Renderable(crnt_renderable_values));
    }

    result
}

fn compile_chunks(chunks: &Vec<CompileChunk>) -> String {
    // The var name is specialified because we don't want someone to call their variable "renderables" then break everything.
    let mut result = "var __spallRenderables = [];\n".to_string();

    for chunk in chunks {
        match chunk {
            CompileChunk::Renderable(renderables) => {
                let simple_renderables = simplify_renderables(&renderables);
                let string_renderables = renderables_to_string(&simple_renderables);
                result += format!("__spallRenderables.push(...[{string_renderables}]);\n").as_str();
            }
            CompileChunk::Javascript(javascript) => {
                result += format!("{javascript}\n").as_str();
            }
        }
    }
    result += "return __spallRenderables;";
    result
}

fn simplify_renderables(renderables: &Vec<Renderable>) -> Vec<Renderable> {
    join_successive_markup_renderables(renderables)
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
                parameters,
            } => {
                new_renderables.push(Renderable::Markup(crnt_markup_string));
                crnt_markup_string = "".to_string();
                new_renderables.push(Renderable::Element {
                    tag_name: tag_name.clone(),
                    compiled_element_name: compiled_element_name.clone(),
                    path: path.clone(),
                    parameters: parameters.clone(),
                });
            }
        }
    }
    if crnt_markup_string.len() > 0 {
        new_renderables.push(Renderable::Markup(crnt_markup_string));
    }

    new_renderables
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
                parameters,
            } => format!(
                r#"new SpallElementRenderable("{tag_name}", {compiled_element_name}, "{path}", {{ {} }})"#,
                parameters
                    .iter()
                    .map(|p| if p.is_dynamic {
                        format!("{}:() => {}", p.name, p.value)
                    } else {
                        format!("{}:() => \"{}\"", p.name, p.value)
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        };
        stringified_renderables.push(string_val);
    }

    stringified_renderables.join(", ")
}
