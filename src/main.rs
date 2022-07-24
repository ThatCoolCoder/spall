mod project_compiler;
mod file_compiler;
mod parser;
mod tag_type;
mod tokeniser;
mod errs;

use crate::errs::*;

fn main() -> Result<(), CompilationError> {
    return project_compiler::compile_project(&std::env::current_dir().unwrap(), false);
}
