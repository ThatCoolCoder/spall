use std::fmt;

#[derive(Clone)]
pub struct TagAttribute {
    pub name: String,
    pub value: String,
    pub is_callback: bool,
}

impl fmt::Display for TagAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_callback {
            write!(f, "{}=\"{}\"(callback) ", self.name, self.value)
        } else {
            write!(f, "{}=\"{}\"", self.name, self.value)
        }
    }
}
