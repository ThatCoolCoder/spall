mod cli;
mod common;
mod compilation_settings;
mod errs;
mod logging;
mod project_compiler;
mod project_compiler_old;
mod scoped_css;
mod spall_markup;

pub fn compile_project(raw_args: &Vec<String>) {
    // Parse args and modify them as needed
    let args = cli::parse_args(raw_args);
    let settings = compilation_settings_from_args(&args);
    let final_path = &std::env::current_dir()
        .unwrap()
        .join(args.project_path)
        .canonicalize()
        .unwrap();

    let result = project_compiler::compile_project(final_path, settings);
    if let Err(e) = result {
        handle_compilation_error(e);
        std::process::exit(1);
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
            other => panic!("Max verbosity is 2 (you said {other})"),
        },
        debug_tokens: args.debug_tokens,
        minify_files: !args.do_not_minify,
        preserve_html_comments: args.preserve_html_comments,
    }
}

fn handle_compilation_error(e: errs::CompilationError) {
    println!("{e}");
}
