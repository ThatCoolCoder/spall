// Converts a .spall file into a javascript file 

use std::path::Path;
use std::fs;

const ROOT_ELEMENT_NAME: &str = "Root";

pub fn compile_element_file(file_path: &Path) -> String {
    // if is not a .spall file: crash

    let file_content = fs::read_to_string(file_path)
        .expect(&format!("Failed reading element file: {}", file_path.to_string_lossy()));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    return compile_element(&file_content, &element_name);
}

pub fn compile_element(file_content: &str, element_name: &str) -> String {
    let uncompiled = file_content.to_owned();
    let compiled_element_name = generate_compiled_element_name(element_name);
    let base_class = if element_name == ROOT_ELEMENT_NAME { "SpallRootElement" } else { "SpallElement" };

    let result = format!(r#"
        class {compiled_element_name} extends {base_class} {{
            generateRenderables() {{
                return ["{uncompiled}"];
            }}
        }}
    "#);

    return result;
}

fn generate_compiled_element_name(element_name: &str) -> String {
    return format!("__SpallCompiled{element_name}");
} 