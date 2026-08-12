use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARNING",
                LogLevel::Error => "ERROR",
            }
        )
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
