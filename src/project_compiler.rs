use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use fs_extra;
use include_dir::{include_dir, Dir, DirEntry};
use itertools::Itertools;
use minifier;

use crate::compilation_settings::*;
use crate::errs;
use crate::file_compiler;
use crate::logging;

#[allow(dead_code)]
struct ProjectPaths {
    root_dir: PathBuf,
    build_dir: PathBuf,
    build_scripts_dir: PathBuf,
    meta_dir: PathBuf,
    elements_dir: PathBuf,
    pages_dir: PathBuf,
    common_dir: PathBuf,
    static_dir: PathBuf,
    build_static_dir: PathBuf,
}

impl ProjectPaths {
    pub fn new(project_dir: &Path) -> ProjectPaths {
        ProjectPaths {
            root_dir: project_dir.to_path_buf(),
            build_dir: project_dir.join("build"),
            build_scripts_dir: project_dir.join("build/scripts"),
            meta_dir: project_dir.join("meta"),
            elements_dir: project_dir.join("elements"),
            pages_dir: project_dir.join("pages"),
            common_dir: project_dir.join("common"),
            static_dir: project_dir.join("static"),
            build_static_dir: project_dir.join("build/static"),
        }
    }
}

const FRAMEWORK_RUNTIME_FILES: Dir = include_dir!("runtime");

pub fn compile_project(
    project_dir: &Path,
    compilation_settings: CompilationSettings,
) -> Result<(), errs::CompilationError> {
    // Entry point of the compiler

    logging::log_always(format!("Compiling project {}", project_dir.to_string_lossy()).as_str());

    logging::log_brief("Preparing for compilation", compilation_settings.log_level);

    let project_paths = ProjectPaths::new(project_dir);

    check_required_dirs_exist(&project_paths)?;

    logging::log_per_step("Setting up build directory", compilation_settings.log_level);
    setup_build_dir(&project_paths);
    copy_index_file(&project_paths)?;
    copy_static_files(&project_paths);

    logging::log_per_step(
        "Building and saving runtime",
        compilation_settings.log_level,
    );
    let mut runtime = build_framework_runtime();
    if compilation_settings.minify_files {
        runtime = minifier::js::minify(&runtime).to_string();
    }
    write_framework_runtime(&project_paths, &runtime);

    logging::log_brief(
        "Compiling elements and pages",
        compilation_settings.log_level,
    );
    let mut compiled_files = compile_elements(
        &project_paths.elements_dir,
        &compilation_settings,
        file_compiler::ElementType::Basic,
    )?;
    compiled_files.extend(compile_elements(
        &project_paths.pages_dir,
        &compilation_settings,
        file_compiler::ElementType::Page,
    )?);
    compiled_files.extend(compile_common_files(&project_paths));

    logging::log_brief("Bundling application", compilation_settings.log_level);
    let mut bundle = bundle_compiled_files(&compiled_files);
    if compilation_settings.minify_files {
        bundle = minifier::js::minify(&bundle).to_string();
    }
    save_bundle(&project_paths, &bundle);

    Ok(())
}

fn check_required_dirs_exist(project_paths: &ProjectPaths) -> Result<(), errs::CompilationError> {
    if !project_paths.elements_dir.exists() {
        Err(errs::CompilationError::Project(
            errs::ProjectCompilationError::NoElementsDirectory,
        ))
    } else if !project_paths.meta_dir.exists() {
        Err(errs::CompilationError::Project(
            errs::ProjectCompilationError::NoMetaDirectory,
        ))
    } else {
        Ok(())
    }
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

fn copy_index_file(project_paths: &ProjectPaths) -> Result<(), errs::CompilationError> {
    fs::copy(
        project_paths.meta_dir.join("index.html"),
        project_paths.build_dir.join("index.html"),
    )
    .map(|_| ())
    .or(Err(errs::CompilationError::Project(
        errs::ProjectCompilationError::NoMetaIndex,
    )))
}

fn copy_static_files(project_paths: &ProjectPaths) {
    // Clean existing directory
    if project_paths.build_static_dir.exists() {
        (if project_paths.build_static_dir.is_dir() {
            fs::remove_dir_all(&project_paths.build_static_dir)
        } else {
            fs::remove_file(&project_paths.build_static_dir)
        })
        .expect("Failed to delete old static directory");
    }
    // Copy files if exists
    if project_paths.build_dir.exists() {
        let mut options = fs_extra::dir::CopyOptions::new();
        // options.mirror_copy = true;
        options.copy_inside = true;
        fs_extra::dir::copy(
            &project_paths.static_dir,
            &project_paths.build_static_dir,
            &options,
        )
        .expect("Failed copying static directory");
    }
}

fn build_framework_runtime() -> String {
    // Yes I know it's not efficient to build it each time, but it would be very painful to do this all through macros.
    // Admittedly pretty shoddy, especially the dependency order part

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

    let mut file_order: Vec<String> = vec![];
    for tuple in &file_map {
        let deps = parse_framework_file_dependencies(&tuple.1);
        add_dependencies_for_file(&deps, &file_map, &mut file_order);
        file_order.push(tuple.0.to_string());
    }

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
    fs::write(
        project_paths.build_dir.join("scripts/framework.js"),
        framework_runtime,
    )
    .expect("Error copying framework scripts");
}
fn compile_elements(
    element_directory: &Path,
    compilation_settings: &CompilationSettings,
    element_types: file_compiler::ElementType,
) -> Result<Vec<String>, errs::CompilationError> {
    // Compile all the elements in the folder as element_types elements

    let element_files = fs::read_dir(element_directory).unwrap();
    let mut compiled_elements = Vec::new();

    for entry in element_files {
        let p = &entry.unwrap().path();
        let file_name = p.as_path();
        compiled_elements.push(
            file_compiler::compile_element_file(
                file_name,
                compilation_settings,
                element_types.clone(),
            )
            .or_else(|e| {
                Err(errs::CompilationError::File {
                    file_name: file_name.to_string_lossy().to_string(),
                    inner_error: e,
                })
            })?,
        );
    }
    Ok(compiled_elements)
}

fn compile_common_files(project_paths: &ProjectPaths) -> Vec<String> {
    if project_paths.common_dir.exists() {
        let common_files = fs::read_dir(&project_paths.common_dir).unwrap();
        common_files
            .map(|entry| {
                let p = &entry.unwrap().path();
                fs::read_to_string(p).unwrap()
            })
            .collect()
    } else {
        vec![]
    }
}

fn bundle_compiled_files(compiled_files: &Vec<String>) -> String {
    compiled_files.join(";\n") // minifier gets a bit too excited if we don't have semicolons after some lines, so add extra ones.
}

fn save_bundle(project_paths: &ProjectPaths, bundle: &str) {
    let target_file = project_paths.build_scripts_dir.join("bundle.js");
    fs::write(target_file, bundle).expect("Failed saving bundle");
}
