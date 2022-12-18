use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use fs_extra;
use include_dir::{include_dir, Dir, DirEntry};
use itertools::Itertools;
use minifier;

use crate::compilation_settings::*;
use crate::errs;
use crate::logging;
use crate::scoped_css;
use crate::spall_markup::{self, element_compiler};

// WIP new project compiler

#[allow(dead_code)]
struct ProjectPaths {
    // Structure providing single point of truth for where the names
    root_dir: PathBuf,
    meta_dir: PathBuf,
    elements_dir: PathBuf,
    pages_dir: PathBuf,
    common_dir: PathBuf,
    static_dir: PathBuf,

    build_dir: PathBuf,
    build_static_dir: PathBuf,
    build_scripts_dir: PathBuf,
}

impl ProjectPaths {
    pub fn new(project_dir: &Path) -> ProjectPaths {
        ProjectPaths {
            root_dir: project_dir.to_path_buf(),
            meta_dir: project_dir.join("meta"),
            elements_dir: project_dir.join("elements"),
            pages_dir: project_dir.join("pages"),
            common_dir: project_dir.join("common"),
            static_dir: project_dir.join("static"),

            build_dir: project_dir.join("build"),
            build_static_dir: project_dir.join("build/static"),
            build_scripts_dir: project_dir.join("build/scripts"),
        }
    }
}

pub fn compile_project(
    project_dir: &Path,
    compilation_settings: CompilationSettings,
) -> Result<(), errs::CompilationError> {
    // Entry point of the compiler

    // devnote: note that there's a lot of logging in here, to some degree I've used logging as stubs+placeholders

    logging::log_always(format!("Compiling project {}", project_dir.to_string_lossy()).as_str());

    logging::log_brief("Preparing for compilation", compilation_settings.log_level);

    let project_paths = ProjectPaths::new(project_dir);

    let index = indexing::index_project(&compilation_settings, &project_paths);

    logging::log_brief("Executing modules:", compilation_settings.log_level);
    logging::log_brief(
        "Initializing build directory",
        compilation_settings.log_level,
    );
    logging::log_brief(
        "Initializing build directory",
        compilation_settings.log_level,
    );

    Ok(())
}

mod indexing {
    use super::*;

    // note: view directory = directory containing markup files + scoped css files (so pages/ and elements/)

    pub type RecursiveDirectoryIndex = HashMap<PathLikeObject, String>;

    #[derive(Default)]
    pub struct ProjectIndex {
        pub page_directory: ViewDirectoryIndex,
        pub element_directory: ViewDirectoryIndex,
        pub common_directory: RecursiveDirectoryIndex,
    }

    #[derive(Default)]
    pub struct ViewDirectoryIndex {
        pub spall_files: RecursiveDirectoryIndex,
        pub scoped_css_files: RecursiveDirectoryIndex,
    }

    pub type PathLikeObject = Vec<String>;

    pub(super) fn index_project(
        compilation_settings: &CompilationSettings,
        project_paths: &ProjectPaths,
    ) -> Result<ProjectIndex, errs::ProjectCompilationError> {
        logging::log_brief("Indexing", compilation_settings.log_level);

        let mut project_index = ProjectIndex::default();

        logging::log_per_step("Indexing element directory", compilation_settings.log_level);
        project_index.element_directory = index_view_directory(&project_paths.elements_dir)?;
        logging::log_per_step("Indexing page directory", compilation_settings.log_level);
        project_index.element_directory = index_view_directory(&project_paths.pages_dir)?;
        logging::log_per_step("Indexing common directory", compilation_settings.log_level);

        Ok(project_index)
    }

    fn index_view_directory(
        view_directory_path: &Path,
    ) -> Result<ViewDirectoryIndex, errs::ProjectCompilationError> {
        // Index one of the view directories

        let mut index = ViewDirectoryIndex::default();
        index_directory(view_directory_path, &mut |path| {
            if let Some(extension) = path.extension() {
                let extension_string = extension.to_string_lossy().to_string();
                let mut insert_into = None;
                if extension_string == ".spall" {
                    insert_into = Some(&mut index.spall_files);
                }
                if extension_string == ".css" {
                    insert_into = Some(&mut index.scoped_css_files);
                }

                if let Some(insert_into) = insert_into {
                    let path_sections: Vec<_> = path
                        .components()
                        .map(|c| match c {
                            std::path::Component::Prefix(_) => None,
                            std::path::Component::RootDir => None,
                            std::path::Component::CurDir => None,
                            std::path::Component::ParentDir => None,
                            std::path::Component::Normal(val) => {
                                Some(val.to_string_lossy().to_string())
                            }
                        })
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .collect();

                    insert_into.insert(path_sections, path.to_string_lossy().to_string());
                }
            }
        });

        Ok(index)
    }

    fn index_common_directory() {}

    fn index_directory(
        directory_path: &Path,
        callback: &mut dyn FnMut(&Path),
    ) -> Result<(), errs::ProjectCompilationError> {
        let entries = match std::fs::read_dir(directory_path) {
            Ok(entries) => entries,
            Err(e) => Err(errs::ProjectCompilationError::ErrorIndexingDirectory {
                directory: directory_path.to_string_lossy().to_string(),
                reason: e.to_string(),
            })?,
        };
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    index_directory(&entry.path(), callback)?;
                } else {
                    callback(&entry.path());
                }
            }
        }
        Ok(())
    }
}
