pub fn run_project(raw_args: &Vec<String>) {
    maybe_show_help_and_exit(raw_args);

    let (build_args, serve_args) = separate_args(raw_args);

    spallcomp::compile_project(&build_args);
    spallserve::serve_project(&serve_args);
}

fn maybe_show_help_and_exit(raw_args: &Vec<String>) {
    match raw_args.first() {
        Some(arg) => match arg.as_str() {
            "-h" | "--help" => {
                show_help();
                std::process::exit(0);
            }
            _ => (),
        },
        None => (),
    }
}

fn show_help() {
    println!("Build then serve a project - intended for development use");
    println!("Usage:\n");
    println!("  spall run -- [args for spall build] -- [args for spall serve] ");
}

fn separate_args(raw_args: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut raw_args = raw_args.clone();
    raw_args.remove(0); // remove default program name from args

    let mut build_args = vec!["spall build".to_string()];
    let mut serve_args = vec!["spall serve".to_string()];

    let mut separator_count = 0;
    for arg in raw_args {
        if arg == "--" {
            separator_count += 1;
            continue;
        }

        match separator_count {
            1 => build_args.push(arg.clone()),
            2 => serve_args.push(arg.clone()),
            _ => break,
        }
    }

    (build_args, serve_args)
}
