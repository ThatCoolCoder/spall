pub mod element_compiler;
mod element_metadata;
mod parser;
mod tag_attribute;
mod tag_type;
mod text_generation;
mod tokeniser;
mod tree_analysis;

#[derive(Clone, PartialEq)]
pub enum ElementType {
    Basic,
    Page,
}