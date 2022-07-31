use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{include_dir, Dir, DirEntry};
use minifier;

use crate::compilation_settings::*;
use crate::errs::*;
use crate::file_compiler;
use crate::logging;

struct ProjectPaths {
    root_dir: PathBuf,
    build_dir: PathBuf,
    build_scripts_dir: PathBuf,
    meta_dir: PathBuf,
    elements_dir: PathBuf,
}

const FRAMEWORK_RUNTIME_FILES: Dir = include_dir!("runtime");

pub fn compile_project(
    project_dir: &Path,
    compilation_settings: CompilationSettings,
) -> Result<(), CompilationError> {
    // Entry point of the compiler

    logging::log_always(format!("Compiling project {}", project_dir.to_string_lossy()).as_str());

    logging::log_brief("Preparing for compilation", compilation_settings.log_level);

    let project_paths = find_project_paths(project_dir);

    logging::log_per_step("Setting up build directory", compilation_settings.log_level);
    setup_build_dir(&project_paths);
    copy_index_file(&project_paths);

    logging::log_per_step(
        "Building and saving runtime",
        compilation_settings.log_level,
    );
    let runtime = build_framework_runtime();
    write_framework_runtime(&project_paths, &runtime);

    logging::log_brief("Compiling elements", compilation_settings.log_level);
    let compiled_elements = compile_elements(&project_paths, &compilation_settings)?;

    logging::log_brief("Bundling application", compilation_settings.log_level);
    let mut bundle = bundle_compiled_elements(&compiled_elements);
    if compilation_settings.minify_bundle {
        bundle = minify_bundle(&bundle);
    }
    save_bundle(&project_paths, &bundle);

    return Ok(());
}

fn find_project_paths(project_dir: &Path) -> ProjectPaths {
    return ProjectPaths {
        root_dir: project_dir.to_path_buf(),
        build_dir: project_dir.join("build"),
        build_scripts_dir: project_dir.join("build/scripts"),
        meta_dir: project_dir.join("meta"),
        elements_dir: project_dir.join("elements"),
    };
}

fn setup_build_dir(project_paths: &ProjectPaths) {
    if !project_paths.build_dir.is_dir() {
        fs::create_dir(&project_paths.build_dir).expect("Failed to create build directory");
    }
    if !project_paths.build_scripts_dir.is_dir() {
        fs::create_dir(&project_paths.build_scripts_dir)
            .expect("Failed creating build scripts dir");
    }
}

fn copy_index_file(project_paths: &ProjectPaths) {
    fs::copy(
        project_paths.meta_dir.join("index.html"),
        project_paths.build_dir.join("index.html"),
    )
    .expect("Error copying index file");
}

fn build_framework_runtime() -> String {
    // Yes I know it's not efficient to build it each time, but it would be very painful to do this through macros.

    return FRAMEWORK_RUNTIME_FILES
        .find("**/*.js")
        .unwrap()
        // Convert entries to none if they are a dir
        .filter_map(|entry| match entry {
            DirEntry::File(f) => Some(f.contents_utf8()),
            _ => None,
        })
        //
        .map(|x| x.unwrap())
        .collect::<Vec<&str>>()
        .join("\n");
}

fn write_framework_runtime(project_paths: &ProjectPaths, framework_runtime: &str) {
    fs::write(
        project_paths.build_dir.join("scripts/framework.js"),
        framework_runtime,
    )
    .expect("Error copying framework scripts");
}

fn compile_elements(
    project_paths: &ProjectPaths,
    compilation_settings: &CompilationSettings,
) -> Result<Vec<String>, CompilationError> {
    // Compile all the elements in the project

    let element_files = fs::read_dir(&project_paths.elements_dir).unwrap();
    let mut compiled_elements = Vec::new();

    for entry in element_files {
        compiled_elements.push(file_compiler::compile_element_file(
            &entry.unwrap().path().as_path(),
            compilation_settings,
        )?);
    }
    return Ok(compiled_elements);
}

fn bundle_compiled_elements(compiled_elements: &Vec<String>) -> String {
    return compiled_elements.join("\n");
}

fn minify_bundle(bundle: &str) -> String {
    return minifier::js::minify(bundle).to_string();
}

fn save_bundle(project_paths: &ProjectPaths, bundle: &str) {
    let target_file = project_paths.build_scripts_dir.join("bundle.js");
    fs::write(target_file, bundle).expect("Failed saving bundle");
}
