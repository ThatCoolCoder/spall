use std::str::FromStr;

use argparse::*;

mod spallrun;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Command {
    build,
    serve,
    run,
    init,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "build" => Ok(Command::build),
            "serve" => Ok(Command::serve),
            "run" => Ok(Command::run),
            "init" => Ok(Command::init),
            _ => Err(()),
        };
    }
}

fn main() {
    let mut subcommand = Command::build;
    let mut args: Vec<String> = vec![];
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.refer(&mut subcommand).required().add_argument(
            "command",
            Store,
            r#"Command to run. Available options are build, serve and run. For more information on specific commands, run spall [COMMAND] --help"#,
        );
        ap.refer(&mut args)
            .add_argument("arguments", List, r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }

    args.insert(0, format!("spall {subcommand:?}"));

    match subcommand {
        Command::build => spallcomp::compile_project(&args),
        Command::serve => spallserve::serve_project(&args),
        Command::run => spallrun::run_project(&args),
        Command::init => spallinit::initialize_project(),
    }
}
