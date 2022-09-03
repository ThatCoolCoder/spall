// Command line interface for program - arg parsing

use argparse;

use crate::compilation_settings::{CompilationLogLevel, CompilationSettings};

pub struct Options {
    pub verbosity: i32,
    pub debug_tokens: bool,
    pub do_not_minify: bool,
    pub project_path: String,
}

pub fn parse_args() -> Options {
    let mut options = Options {
        verbosity: 0,
        debug_tokens: false,
        do_not_minify: false,
        project_path: "".to_string(),
    };

    {
        let mut parser = argparse::ArgumentParser::new();
        parser.refer(&mut options.verbosity).add_option(
            &["-v", "--verbose"],
            argparse::IncrBy(1),
            "Verbosity. Has three levels, signified by repeating the argument.",
        );
        parser.refer(&mut options.debug_tokens).add_option(
            &["-t", "--token-debug"],
            argparse::StoreTrue,
            "Whether to print tokens for debugging purposes",
        );
        parser.refer(&mut options.do_not_minify).add_option(
            &["-l", "--large"],
            argparse::StoreTrue,
            "Whether to disable minifying of the final bundle for debugging purposes",
        );
        parser.refer(&mut options.project_path).add_option(
            &["-p", "--project"],
            argparse::Store,
            "Path to project to compile",
        );
        parser.parse_args_or_exit();
    }
    options
}
