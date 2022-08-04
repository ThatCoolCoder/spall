mod compilation_settings;
mod errs;
mod file_compiler;
mod javascript_type;
mod logging;
mod parser;
mod project_compiler;
mod tag_attribute;
mod tag_type;
mod tokeniser;

fn main() -> Result<(), errs::CompilationError> {
    return project_compiler::compile_project(
        &std::env::current_dir().unwrap(),
        compilation_settings::CompilationSettings {
            log_level: compilation_settings::CompilationLogLevel::PerStep,
            minify_bundle: false,
        },
    );
}
