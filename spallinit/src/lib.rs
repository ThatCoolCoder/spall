use std::io::Write;

use include_dir::{include_dir, Dir};

const PROJECT_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/project_template");

pub fn initialize_project() {
    if !confirmed_wants_to_proceed() {
        println!("Cancelling...");
        return;
    }

    match write_template_directory() {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to create project: {e}")
        }
    }

    println!("Done");
}

fn confirmed_wants_to_proceed() -> bool {
    let stdin = std::io::stdin();
    println!("spall init will initialize a new spall project in the current directory.");
    print!("This will likely result in IRREVERSIBLE MODIFICATIONS OR DELETION to anything already there. Proceed? (y/N) ");
    std::io::stdout().flush().expect("Error flushing stdout"); // why does print! not do this already?

    let mut answer = "".to_string();
    stdin
        .read_line(&mut answer)
        .expect("Error reading standard input");

    return answer.chars().next().unwrap_or('n') == 'y';
}

fn write_template_directory() -> Result<(), std::io::Error> {
    PROJECT_TEMPLATE.extract(std::env::current_dir().expect("Failed finding current dir somehow"))
}
