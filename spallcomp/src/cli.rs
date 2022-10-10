// Command line interface for program - arg parsing

use argparse;

// Serves as an intermediate step between command line and CompilationSettings, since a direct mapping would not be suitable or flexible
pub struct Options {
    pub verbosity: i32,
    pub debug_tokens: bool,
    pub do_not_minify: bool,
    pub project_path: String,
    pub preserve_html_comments: bool,
}

pub fn parse_args(args: &Vec<String>) -> Options {
    // Convert command line args into an Options struct

    // Init default options
    let mut options = Options {
        verbosity: 0,
        debug_tokens: false,
        do_not_minify: false,
        project_path: "".to_string(),
        preserve_html_comments: false,
    };

    // Set up argparser and use it
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
        parser.refer(&mut options.project_path).add_argument(
            "project",
            argparse::Store,
            "Path to project to compile",
        );
        parser
            .refer(&mut options.preserve_html_comments)
            .add_option(
                &["-c", "--comments"],
                argparse::StoreTrue,
                "Preserve HTML comments in final markup",
            );
        let result = parser.parse(args.clone(), &mut std::io::stdout(), &mut std::io::stderr());
        if let Err(err_code) = result {
            println!("");
            std::process::exit(err_code);
            // parser.print_usage("spallcomp", &mut std::io::stdout());
        }
    }
    options
}
