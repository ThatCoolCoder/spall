use crate::compilation_settings::CompilationLogLevel;

pub fn log_always(data: &str) {
    log(
        data,
        CompilationLogLevel::Minimal,
        CompilationLogLevel::Minimal,
    );
}

pub fn log_brief(data: &str, log_level: CompilationLogLevel) {
    log(data, log_level, CompilationLogLevel::Brief);
}

pub fn log_per_step(data: &str, log_level: CompilationLogLevel) {
    log(data, log_level, CompilationLogLevel::PerStep);
}

fn log(data: &str, log_level: CompilationLogLevel, required_log_level: CompilationLogLevel) {
    if log_level >= required_log_level {
        println!("{}", data);
    }
}
