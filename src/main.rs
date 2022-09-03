mod cli;
mod compilation_settings;
mod errs;
mod file_compiler;
mod logging;
mod parser;
mod project_compiler;
mod tag_attribute;
mod tag_type;
mod tokeniser;

fn main() {
    let args = cli::parse_args();
    let mut settings = compilation_settings_from_args(&args);
    settings.minify_bundle = false; // minifying is broken currently
    let final_path = &std::env::current_dir()
        .unwrap()
        .join(args.project_path)
        .canonicalize()
        .unwrap();

    let result = project_compiler::compile_project(final_path, settings);
    if let Err(e) = result {
        handle_compilation_error(e);
    } else {
        println!("Done!");
    }
}

fn compilation_settings_from_args(
    args: &cli::Options,
) -> compilation_settings::CompilationSettings {
    compilation_settings::CompilationSettings {
        log_level: match args.verbosity {
            0 => compilation_settings::CompilationLogLevel::Minimal,
            1 => compilation_settings::CompilationLogLevel::Brief,
            2 => compilation_settings::CompilationLogLevel::PerStep,
            _ => panic!("Max verbosity is 2"),
        },
        debug_tokens: args.debug_tokens,
        minify_bundle: !args.do_not_minify,
    }
}

fn handle_compilation_error(e: errs::CompilationError) {
    println!("{e}");
}
