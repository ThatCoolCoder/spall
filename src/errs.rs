#[derive(Debug)]
pub enum CompilationError {
    InvalidElementName { name: String }
}