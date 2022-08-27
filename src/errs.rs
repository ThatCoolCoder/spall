use std::fmt;

#[derive(Debug)]
pub enum CompilationError {
    Project,
    File {
        file_name: String,
        inner_error: FileCompilationError,
    },
}
impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilationError::Project => write!(
                f,
                "Unknown error compiling project. Likely things haven't been set up properly"
            ),
            CompilationError::File {
                file_name,
                inner_error,
            } => write!(f, "Error compiling {}:\n    {}", file_name, inner_error),
        }
    }
}

#[derive(Debug)]
pub enum FileCompilationError {
    InvalidElementName { name: String },
    MarkupSyntaxError(MarkupSyntaxError),
}

impl fmt::Display for FileCompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileCompilationError::InvalidElementName { name } => {
                write!(f, "Element name \"{name}\" is not valid")
            }
            FileCompilationError::MarkupSyntaxError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum MarkupSyntaxError {
    AttributesOnCloseTag { tag_name: String },
}

impl fmt::Display for MarkupSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let main_text = match self {
            MarkupSyntaxError::AttributesOnCloseTag { tag_name } => {
                format!("Closing HTML tags cannot have attributes (tag name {tag_name})")
            }
        };
        write!(f, "Syntax error in markup. {main_text}")
    }
}
