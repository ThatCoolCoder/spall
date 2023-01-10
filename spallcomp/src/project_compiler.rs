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
    build_css_dir: PathBuf,
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
            build_scripts_dir: project_dir.join("build/static/js"),
            build_css_dir: project_dir.join("build/static/css"),
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

    {
        // Execute modules
        logging::log_brief("Executing modules", compilation_settings.log_level);
        setup_build_directory(&compilation_settings, &project_paths)?;
    }

    Ok(())
}

fn setup_build_directory(
    compilation_settings: &CompilationSettings,
    project_paths: &ProjectPaths,
) -> Result<(), errs::CompilationError> {
    // Perform miscellanious tasks required to set up the build directory

    logging::log_brief("Setting up build directory", compilation_settings.log_level);

    logging::log_per_step("Creating build directory", compilation_settings.log_level);
    if !project_paths.build_dir.is_dir() {
        fs::create_dir(&project_paths.build_dir).expect("Failed to create build directory");
    }

    logging::log_per_step(
        "Creating build scripts directory",
        compilation_settings.log_level,
    );
    if !project_paths.build_scripts_dir.is_dir() {
        fs::create_dir(&project_paths.build_scripts_dir)
            .expect("Failed creating build scripts dir");
    }

    logging::log_per_step(
        "Creating build css directory",
        compilation_settings.log_level,
    );
    if !project_paths.build_css_dir.is_dir() {
        fs::create_dir(&project_paths.build_css_dir).expect("Failed creating build css dir");
    }

    logging::log_per_step(
        "Cleaning build static directory",
        compilation_settings.log_level,
    );
    if project_paths.build_static_dir.exists() {
        (if project_paths.build_static_dir.is_dir() {
            fs::remove_dir_all(&project_paths.build_static_dir)
        } else {
            fs::remove_file(&project_paths.build_static_dir)
        })
        .expect("Failed to delete old static directory");
    }

    logging::log_per_step(
        "Copying over static directory",
        compilation_settings.log_level,
    );
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    fs_extra::dir::copy(
        &project_paths.static_dir,
        &project_paths.build_static_dir,
        &options,
    )
    .expect("Failed copying static directory");

    logging::log_per_step("Copying index file", compilation_settings.log_level);

    Ok(())
}

mod indexing {
    use super::*;

    // note: view directory = directory containing markup files + scoped css files (so pages/ and elements/)

    pub type RecursiveDirectoryIndex = HashMap<SPath, String>;

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

    // "spall path" - I don't know what to call it, it's just a thing representing a path within a namespace system
    pub type SPath = Vec<String>;
    // Macro for constructing a spall path in code.
    macro_rules! spath {
        ( $( $x:expr ),* ) => {
            {
                let mut temp = SPath::new();
                $(
                    temp.push($x.to_string());
                )*
                temp
            }
        };
    }

    pub(super) fn index_project(
        compilation_settings: &CompilationSettings,
        project_paths: &ProjectPaths,
    ) -> Result<ProjectIndex, errs::ProjectCompilationError> {
        logging::log_brief("Indexing", compilation_settings.log_level);

        let mut project_index = ProjectIndex::default();

        logging::log_per_step("Indexing element directory", compilation_settings.log_level);
        project_index.element_directory =
            index_view_directory(&project_paths.elements_dir, spath!("Elements"))?;
        logging::log_per_step("Indexing page directory", compilation_settings.log_level);
        project_index.element_directory =
            index_view_directory(&project_paths.pages_dir, spath!("Pages"))?;
        logging::log_per_step("Indexing common directory", compilation_settings.log_level);
        project_index.common_directory = index_common_directory(&project_paths.common_dir)?;

        Ok(project_index)
    }

    fn index_view_directory(
        view_directory_path: &Path,
        base_spath: SPath,
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
                    let mut spath = path_to_spath(&path);
                    let mut full_path = base_spath.clone();
                    full_path.append(&mut spath);
                    insert_into.insert(full_path, path.to_string_lossy().to_string());
                }
            }
        })?;

        Ok(index)
    }

    fn index_common_directory(
        common_directory_path: &Path,
    ) -> Result<RecursiveDirectoryIndex, errs::ProjectCompilationError> {
        let mut index = RecursiveDirectoryIndex::default();
        index_directory(common_directory_path, &mut |path| {
            if let Some(extension) = path.extension() {
                if extension.to_string_lossy() == ".js" {
                    let mut spath = path_to_spath(&path);
                    let mut full_path = spath!("Common");
                    full_path.append(&mut spath);
                    index.insert(full_path, path.to_string_lossy().to_string());
                }
            }
        })?;
        Ok(index)
    }

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

    fn path_to_spath(path: &Path) -> SPath {
        // Generate an spath based on the components of a path, useful for namespacing
        path.components()
            .map(|c| match c {
                std::path::Component::Prefix(_) => None,
                std::path::Component::RootDir => None,
                std::path::Component::CurDir => None,
                std::path::Component::ParentDir => None,
                std::path::Component::Normal(val) => Some(val.to_string_lossy().to_string()),
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
}
