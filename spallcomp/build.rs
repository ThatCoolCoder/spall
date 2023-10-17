use std::env;
use std::fs;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

const FRAMEWORK_RUNTIME_DIR: &'static str = "test";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=runtime");

    build_and_save_runtime();
}

fn build_and_save_runtime() {
    let runtime = build_framework_runtime();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let target = Path::new(&out_dir).join("runtime_compiled.js");
    fs::write(&target, &runtime).unwrap();
}



fn build_framework_runtime() -> String {
    // Admittedly pretty shoddy, especially the dependency order part

    // Read files from binary, turn them into a map of name to content.
    let file_map: HashMap<String, String> = FRAMEWORK_RUNTIME_FILES
        .find("**/*.js")
        .unwrap()
        .filter_map(|entry| match entry {
            DirEntry::File(f) => Some((
                f.path().to_string_lossy().to_string(),
                f.contents_utf8().unwrap().to_string(),
            )),
            _ => None,
        })
        .collect();

    // Part 1 of determining the order in which the files should be appended to make sure dependencies are fulfilled
    let mut file_order: Vec<String> = vec![];
    for tuple in &file_map {
        let deps = parse_framework_file_dependencies(&tuple.1);
        add_dependencies_for_file(&deps, &file_map, &mut file_order);
        file_order.push(tuple.0.to_string());
    }

    // Part 2 of determining them - remove duplicates.
    // Then processing them to remove the require, and concatenating.
    file_order
        .iter()
        .unique()
        .map(|x| remove_require_statement(file_map.get(x).unwrap()))
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_framework_file_dependencies(file_content: &str) -> Vec<String> {
    // Returns a list of files it depends on and returns a copy of the file with require statement removed
    if file_content.starts_with("requires(") {
        let requirements: Vec<String> = file_content
            .lines()
            .next()
            .unwrap()
            .replacen("requires(", "", 1)
            .replace(");", "")
            .split(",")
            .map(|x| x.trim().to_string())
            .collect();
        requirements
    } else {
        vec![]
    }
}

fn add_dependencies_for_file(
    file_dependencies: &Vec<String>,
    file_map: &HashMap<String, String>,
    dependency_accumulator: &mut Vec<String>,
) {
    // Recursive function to find the dependencies for a file and add them to the accumulator before then adding the file itself.

    for file in file_dependencies {
        if !file_map.contains_key(file) {
            panic!("Could not find file {}", file);
        }
        add_dependencies_for_file(
            &parse_framework_file_dependencies(file_map.get(file).unwrap()),
            file_map,
            dependency_accumulator,
        );
        dependency_accumulator.push(file.to_string());
    }
}

fn remove_require_statement(text: &str) -> String {
    // Remove the requires() statement from the first line of a framework file if it has one
    if text.starts_with("requires(") {
        text.lines().skip(1).collect::<Vec<&str>>().join("\n")
    } else {
        text.to_string()
    }
}

fn write_framework_runtime(project_paths: &ProjectPaths, framework_runtime: &str) {
    // Write compiled framework runtime to where it belongs in the build dir

    fs::write(
        project_paths.build_dir.join("scripts/framework.js"),
        framework_runtime,
    )
    .expect("Error copying framework scripts");
}