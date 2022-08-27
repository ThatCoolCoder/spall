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

fn main() {
    let result = project_compiler::compile_project(
        &std::env::current_dir().unwrap(),
        compilation_settings::CompilationSettings {
            log_level: compilation_settings::CompilationLogLevel::Minimal,
            minify_bundle: false,
            debug_tokens: true,
        },
    );
    if let Err(e) = result {
        handle_compilation_error(e);
    } else {
        println!("Done!");
    }
}

fn handle_compilation_error(e: errs::CompilationError) {
    println!("{e}");
}
