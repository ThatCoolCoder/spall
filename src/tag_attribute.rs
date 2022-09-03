use std::fmt;

#[derive(Clone)]
pub struct TagAttribute {
    pub name: String,
    pub value: String,
    pub is_dynamic: bool,
}

impl fmt::Display for TagAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_dynamic {
            write!(f, "{}=\"{}\"(dynamic) ", self.name, self.value)
        } else {
            write!(f, "{}=\"{}\"", self.name, self.value)
        }
    }
}
