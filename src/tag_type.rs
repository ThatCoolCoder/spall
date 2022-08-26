use derive_more::Display;

#[derive(Display, PartialEq, Debug)]
pub enum TagType {
    Start,      // like <p>
    End,        // like </p>
    Standalone, // like <input />
}
