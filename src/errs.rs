use std::fmt;

use crate::tag_type::TagType;

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
    UnbalancedTag(UnbalancedTag),
    UnbalancedInlineJavascript,
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
            MarkupSyntaxError::UnbalancedInlineJavascript => {
                format!("Unbalanced opening/closing of inline Javascript")
            }
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
