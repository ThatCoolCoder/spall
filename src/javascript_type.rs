use derive_more::Display;

#[derive(Display)]
pub enum JavascriptType {
    BlockStart, // like if (true) {
    BlockEnd,   // like }
    Standalone, // like this.brains = 5;
}
