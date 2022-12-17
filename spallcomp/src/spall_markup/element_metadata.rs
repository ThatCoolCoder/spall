use crate::errs;
use super::ElementType;

const ROOT_ELEMENT_NAME: &str = "Root";

pub struct ElementMetadata {
    pub compiled_element_name: String,
    pub element_base_class: String,
    pub element_type: ElementType
}

pub fn determine_element_metadata(element_name: &str, element_type: ElementType) -> Result<ElementMetadata, errs::FileCompilationError> {
    if !element_name_valid(element_name) {
        return Err(errs::FileCompilationError::InvalidElementName {
            name: element_name.to_owned(),
        });
    }

    let compiled_element_name = generate_compiled_element_name(element_name);
    let element_base_class = find_element_base_class(&element_type, element_name);

    Ok(ElementMetadata {
        compiled_element_name,
        element_base_class,
        element_type
    })
}

pub fn generate_compiled_element_name(element_name: &str) -> String {
    format!("__SpallCompiled{element_name}")
}

fn element_name_valid(element_name: &str) -> bool {
    if element_name.len() == 0 {
        false
    } else if !element_name.chars().next().unwrap().is_alphabetic() {
        false
    } else if element_name.chars().any(|c| !c.is_alphanumeric()) {
        false
    } else {
        true
    }
}

fn find_element_base_class(element_type: &ElementType, element_name: &str) -> String {
    // Find the JS class that the element should extend from

    if *element_type == ElementType::Basic {
        if element_name == ROOT_ELEMENT_NAME {
            "SpallRootElement".to_string()
        } else {
            "SpallElement".to_string()
        }
    } else {
        "SpallPage".to_string()
    }
}