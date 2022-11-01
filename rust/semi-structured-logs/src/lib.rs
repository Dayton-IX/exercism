// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// formatter for enum to string
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR")
        }
    }
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut output = String::new();
    output.push('[');
    output.push_str(level.to_string());
    output.push_str("]: ");
    output.push_str(message);
    return output;
}
pub fn info(message: &str) -> String {
    unimplemented!("return a message for info log level")
}
pub fn warn(message: &str) -> String {
    unimplemented!("return a message for warn log level")
}
pub fn error(message: &str) -> String {
    unimplemented!("return a message for error log level")
}
