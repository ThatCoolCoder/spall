use crate::logging;
use crate::errs;
use crate::compilation_settings::CompilationSettings;
use crate::common::compiler_utils;

use super::ElementType;
use super::element_metadata::{ElementMetadata};
use super::parser;
use super::tree_analysis::{self, CompileChunk, Renderable};

pub fn compile_tree(compilation_settings: &CompilationSettings, metadata: &ElementMetadata,
    tree: &parser::Tree) -> Result<String, errs::FileCompilationError> {
    logging::log_per_step("Actually compiling", compilation_settings.log_level);

    // main fn of mod - convert tree to compiled javascript text

    let chunks = tree_analysis::create_compile_chunks(tree);
    let class_body = find_class_body(&tree).unwrap_or("".to_string());
    let compiled_render_func = compile_chunks(&chunks);

    let extra_methods = compile_extra_methods(&metadata.element_type, &tree);
    // Create main body
    let mut result = format!(
        r#"
        class {compiled_element_name} extends {element_base_class} {{
            constructor(id, parentId, spallApp, path) {{
                super('{element_name}', id, parentId, spallApp, path);
            }}

            compiledGenerateRenderables() {{
                {compiled_render_func}
            }}

            {extra_methods}

            {class_body}
        }}
    "#,
    compiled_element_name = metadata.compiled_element_name,
    element_base_class = metadata.element_base_class,
    element_name = metadata.compiled_element_name);

    if metadata.element_type == ElementType::Page {
        // add code to register as page

        let mut page_routes = vec![];
        // add code to register as page
        tree.depth_first_map(&mut |node, _is_entering| {
            if let parser::NodeData::Markup(inner_data) = &node.data {
                if inner_data.tag_name == "pageroute" {
                    page_routes.push(inner_data.inner_text.clone());
                }
            };
        });

        if page_routes.len() == 0 {
            return Err(errs::FileCompilationError::NoPageRoutes);
        }

        result += &compile_all_page_routes(&page_routes, &metadata.compiled_element_name);
    }

    Ok(result)
}


fn compile_chunks(chunks: &Vec<CompileChunk>) -> String {
    // Actually compile the compilation chunks

    // The var name is made special because we don't want someone to call their variable "renderables" then break everything.
    let mut result = "var __spallRenderables = [];\n".to_string();

    for chunk in chunks {
        match chunk {
            CompileChunk::Renderable(renderables) => {
                let simple_renderables = simplify_renderables(&renderables);
                let string_renderables = compile_renderables(&simple_renderables);
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
    // Perform some simplifications on the renderables. Very basic now, but could be expanded to optimise the code
    
    join_successive_markup_renderables(renderables)
}

fn join_successive_markup_renderables(renderables: &Vec<Renderable>) -> Vec<Renderable> {
    // Join consecutive markup renderables together, EG going from ['<p>', '</p>'] to ['<p></p>']

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



fn compile_renderables(renderables: &Vec<Renderable>) -> String {
    // Convert a set of renderables (EG from a compile chunk) to a string

    let mut stringified_renderables = vec![];
    for renderable in renderables {
        let string_val = match renderable {
            Renderable::Markup(value) => format!(
                "new SpallMarkupRenderable(`{}`)",
                compiler_utils::escape_quotes(&value, '`', '\\')
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

// PAGE STUFF, SHOULD GET RID OF

fn compile_all_page_routes(raw_page_routes: &Vec<String>, element_name: &str) -> String {
    raw_page_routes
        .iter()
        .map(|route| {
            let compiled_route = compile_page_route(route);
            format!("SpallRouter.routeList.push([{compiled_route},{element_name}]);")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn compile_page_route(raw_route: &str) -> String {
    let sections = raw_route.split('/').filter(|x| !x.trim().is_empty());
    // todo: if sections contain ${} but it's not at the start + end then cry.
    // todo: check validity of route name
    let compiled_sections = sections
        .map(|s| {
            if s.starts_with("${") {
                let cleaned = s.replace("${", "").replace("}", "");
                format!("new SpallPropertyRouteSection(\"{cleaned}\")")
            } else {
                format!("new SpallStringRouteSection(\"{s}\")")
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!("[{compiled_sections}]")
}


fn compile_extra_methods(element_type: &ElementType, tree: &parser::Tree) -> std::string::String {
    // Compile extra methods for the element
    // refactoring note: is just a thing to do stuff if it's a page element
    match *element_type {
        ElementType::Basic => "".to_string(),
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
            format!(
                r#"
                generateTitle() {{
                    return `{page_title}`;
                }}"#
            )
        }
    }
}

fn find_class_body(tree: &parser::Tree) -> Option<String> {
    // Find the <script> tag that contains the user-defined body of the element class

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