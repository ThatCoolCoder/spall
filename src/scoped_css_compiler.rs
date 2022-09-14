use std::fs;
use std::path::Path;

pub fn compile_scoped_css_file(file_path: &Path) -> String {
    let file_content = fs::read_to_string(file_path).expect(&format!(
        "Failed reading scoped css file: {}",
        file_path.to_string_lossy()
    ));
    let element_name = file_path.file_stem().unwrap().to_str().unwrap();
    compile_scoped_css(&file_content, &element_name)
}

pub fn compile_scoped_css(file_content: &str, element_name: &str) -> String {
    "".to_string()
}
