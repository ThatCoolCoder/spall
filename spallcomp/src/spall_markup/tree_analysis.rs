// Handles turning the node tree into something that 

use super::tag_attribute::TagAttribute;
use super::element_metadata;
use super::parser;

// Elements that aren't put into the final markup
const IGNORED_ELEMENT_NAMES: [&'static str; 3] = ["title", "pageroute", "script"];

pub enum CompileChunk {
    // Chunk of stuff that we need to compile
    Javascript(String),
    Renderable(Vec<Renderable>),
}

#[derive(Clone)]
pub enum Renderable {
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
pub struct ElementParameter {
    pub name: String,
    pub value: String,
    pub is_dynamic: bool, // whether it is a plain text value or is executed at render-time.
                      // Dynamic parameters are those which have a ! at the start in the initial HTML,
                      // although when they are put into the structure the ! is stripped
}



// A note on the structure of the output:
// The output consists of compile chunks, which correspond to sections of the Javascript output.
// Chunks can be interpolated javascript (which is pasted directly into the output), or lists of renderables
// Renderables correspond to the javascript class of the same name - they are something that is handed off the framework from the compiled element, processed, then injected into the DOM.
// A renderable instruct either to generate markup (raw HTML), or to instantiate a spall element
// Note that part of generating markup renderables involves compiling the node into text - this isn't *really* considered text generation so it's in this module

pub fn create_compile_chunks(tree: &parser::Tree) -> Vec<CompileChunk> {
    // main fn of mod - convert node tree into list of chunks, ready for final compilation

    let chunks = compile_chunks_from_tree(&tree);
    concat_successive_compile_chunks(&chunks)
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
                compiled_element_name: element_metadata::generate_compiled_element_name(&node_data.tag_name),
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



fn compile_tag_attributes(tag_attributes: &Vec<TagAttribute>, _tag_path: &str) -> String {
    tag_attributes
        .iter()
        .map(|x| {
            // for this.x() callbacks, get context for the "this" by lookups through the renderer
            if x.is_dynamic && x.value.starts_with("this.") {
                let this_removed = x.value.replacen("this.", "", 1);
                // take advantage of the way that strings are inserted into js to inject some stuff from runtime into the html
                format!(
                    "{}=\"SpallApp.instance.renderer.getElementById(${{this.id}}).{}\"",
                    x.name, this_removed
                )
            } else {
                format!("{}=\"{}\"", x.name, x.value)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}