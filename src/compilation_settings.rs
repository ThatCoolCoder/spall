pub struct CompilationSettings {
    pub log_level: CompilationLogLevel,
    pub minify_files: bool,
    pub debug_tokens: bool,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum CompilationLogLevel {
    Minimal = 1, // basically no logging
    Brief = 2,   // log that you're compiling each file
    PerStep = 3, // log each step of compiling each file
}
