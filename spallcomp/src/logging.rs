use crate::compilation_settings::CompilationLogLevel;

pub fn log_always(data: &str) {
    // Log something that should always be logged, such as what project we're doing

    log(
        data,
        CompilationLogLevel::Minimal,
        CompilationLogLevel::Minimal,
    );
}

pub fn log_brief(data: &str, log_level: CompilationLogLevel) {
    // Log something that might be wanted for minor debugging such as what step or file we're working on
    log(data, log_level, CompilationLogLevel::Brief);
}

pub fn log_per_step(data: &str, log_level: CompilationLogLevel) {
    // Log really detailed stuff for in-depth debugging
    log(data, log_level, CompilationLogLevel::PerStep);
}

fn log(data: &str, log_level: CompilationLogLevel, required_log_level: CompilationLogLevel) {
    // Base log method, consumers should use the other ones so you don't have to specify level of logging

    if log_level >= required_log_level {
        println!("{}", data);
    }
}
