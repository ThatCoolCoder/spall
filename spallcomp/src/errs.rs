use std::fmt;

#[derive(Debug)]
pub enum CompilationError {
    Project(ProjectCompilationError),
    File {
        file_name: String,
        inner_error: FileCompilationError,
    },
}
impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilationError::Project(inner_error) => {
                write!(f, "Error compiling project:\n    {inner_error}")
            }
            CompilationError::File {
                file_name,
                inner_error,
            } => write!(f, "Error compiling {}:\n    {}", file_name, inner_error),
        }
    }
}

#[derive(Debug)]
pub enum ProjectCompilationError {
    NoElementsDirectory,
    ErrorIndexingDirectory { directory: String, reason: String },
    NoMetaDirectory,
    NoRootElement,
    NoMetaIndex,
}

impl fmt::Display for ProjectCompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectCompilationError::NoElementsDirectory => {
                write!(f, "Could not find elements/ directory, are you sure there is a spall project located here?")
            }
            ProjectCompilationError::ErrorIndexingDirectory { directory, reason } => {
                write!(f, "Error indexing {}: {}", directory, reason)
            }
            ProjectCompilationError::NoMetaDirectory => write!(f, "Could not find meta/ directory, are you sure there is a spall project located here?"),
            ProjectCompilationError::NoRootElement => {
                write!(f, "No root element (elements/Root.spall) defined.")
            }
            ProjectCompilationError::NoMetaIndex => write!(f, "No index.html defined in meta/ dir"),
        }
    }
}

#[derive(Debug)]
pub enum FileCompilationError {
    InvalidElementName { name: String },
    NoPageRoutes,
    CssSyntaxError(CssSyntaxError),
    MarkupSyntaxError(MarkupSyntaxError),
}

impl fmt::Display for FileCompilationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileCompilationError::InvalidElementName { name } => {
                write!(f, "Element name \"{name}\" is not valid")
            }
            FileCompilationError::NoPageRoutes => {
                write!(f, "No page route was defined")
            }
            FileCompilationError::CssSyntaxError(e) => e.fmt(f),
            FileCompilationError::MarkupSyntaxError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum CssSyntaxError {
    UnexpectedEndOfFile,
}

impl fmt::Display for CssSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssSyntaxError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
        }
    }
}

#[derive(Debug)]
pub enum MarkupSyntaxError {
    AttributesOnCloseTag { tag_name: String },
    UnbalancedTag(UnbalancedTag),
    OrphanedNode,
    UnmatchedTokenTypes,
}

impl fmt::Display for MarkupSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let main_text = match self {
            MarkupSyntaxError::AttributesOnCloseTag { tag_name } => {
                format!("Closing HTML tags cannot have attributes (tag name {tag_name})")
            }
            MarkupSyntaxError::UnbalancedTag(inner_data) => format!("{inner_data}"),
            MarkupSyntaxError::OrphanedNode => {
                format!("Found orphaned node - all nodes must have a parent, except for the root")
            }
            MarkupSyntaxError::UnmatchedTokenTypes => {
                format!("Token types do not match (have you missed an open/close tag?)")
            }
        };
        write!(f, "Syntax error in markup. {main_text}")
    }
}

#[derive(Debug)]
pub enum UnbalancedTag {
    UnmatchingNames {
        start_tag_name: String,
        end_tag_name: String,
    },
    UnclosedStartTag {
        tag_name: String,
    },
    UnopenedEndTag {
        tag_name: String,
    },
}

impl fmt::Display for UnbalancedTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnbalancedTag::UnmatchingNames {
                start_tag_name,
                end_tag_name,
            } => {
                write!(f, "Unmatched tag names: opening tag was a \"{start_tag_name}\" but closing tag was a \"{end_tag_name}\"")
            }
            UnbalancedTag::UnclosedStartTag { tag_name } => {
                write!(f, "Tag \"{tag_name}\" was not closed")
            }
            UnbalancedTag::UnopenedEndTag { tag_name } => {
                write!(f, "End tag \"{tag_name}\" has no matching start tag ")
            }
        }
    }
}
