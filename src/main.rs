mod project_compiler;
mod file_compiler;
mod parser;
mod tokeniser;

fn main() {
    project_compiler::compile_project(&std::env::current_dir().unwrap());
}
