mod compile_project;
mod compile_file;

fn main() {
    compile_project::compile_project(&std::env::current_dir().unwrap());
}
