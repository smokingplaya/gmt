use colored::Colorize;

#[allow(dead_code)]
pub enum LogType {
    Error,
    Warning,
    Info
}

const PREFIX: &'static str = "|";

fn output(prefix: String, message: &str) {
    println!("{prefix} {message}");
}

pub fn print(log_type: LogType, message: &str) {
    match log_type {
        LogType::Error => output(PREFIX.red().to_string(), message),
        LogType::Warning => output(PREFIX.yellow().to_string(), message),
        LogType::Info => output(PREFIX.blue().to_string(), message),
    }
}