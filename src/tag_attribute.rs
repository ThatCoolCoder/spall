use derive_more::Display;

#[derive(Clone, Display)]
#[display(fmt = "{name}=\"{value}\"")]
pub struct TagAttribute {
    pub name: String,
    pub value: String,
}
