use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use fs_extra;
use include_dir::{include_dir, Dir, DirEntry};
use itertools::Itertools;
use minifier;

use crate::compilation_settings::*;
use crate::spall_markup::{self, element_compiler};
use crate::errs;
use crate::logging;
use crate::scoped_css;

// WIP new project compiler

#[allow(dead_code)]
struct ProjectPaths {
    // Structure providing single point of truth for where the names 
    root_dir: PathBuf,
    build_dir: PathBuf,
    build_scripts_dir: PathBuf,
    meta_dir: PathBuf,
    elements_dir: PathBuf,
    pages_dir: PathBuf,
    common_dir: PathBuf,
    static_dir: PathBuf,
    build_static_dir: PathBuf,
    scoped_css_dir: PathBuf,
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
            scoped_css_dir: project_dir.join("styles"),
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
    
    let index = indexing::index_project(&compilation_settings);

    logging::log_brief("Executing modules:", compilation_settings.log_level);
    logging::log_brief("Initializing build directory", compilation_settings.log_level);
    logging::log_brief("Initializing build directory", compilation_settings.log_level);
    
    
    Ok(())
}

mod indexing {
    use super::*;

    // note: view directory = directory containing markup files + scoped css files (so pages/ and elements/)

    pub struct ProjectIndex {
        pub page_directory: ViewDirectoryIndex,
        pub element_directory: ViewDirectoryIndex,
        // pub common_directory: map of PathLikeObject to filename
    }

    pub struct ViewDirectoryIndex {
        // pub spallFiles: map of PathLikeObject to filename
        // pub scopedCssFiles: map of PathLikeObject to filename
    }

    pub struct PathLikeObject {
        // represents a filename processed into a list of sub-namespaces or whatever
    }

pub fn index_project(compilation_settings: &CompilationSettings) {
    logging::log_brief("Indexing", compilation_settings.log_level);

    logging::log_per_step("Indexing element directory", compilation_settings.log_level);
    logging::log_per_step("Indexing page directory", compilation_settings.log_level);
    logging::log_per_step("Indexing common directory", compilation_settings.log_level);

}

fn index_view_directory() {
    // Index view 
}

    fn index_common_directory() {

    }
}