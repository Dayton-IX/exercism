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
// impl fmt::Display for LogLevel {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             LogLevel::Info => write!(f, "INFO"),
//             LogLevel::Warning => write!(f, "WARNING"),
//             LogLevel::Error => write!(f, "ERROR")
//         }
//     }
// }
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // TODO if else to call other functions depending on level
}
pub fn info(message: &str) -> String {
    let mut output = String::from("[INFO]: ");
    output.push_str(message);
    return output;
}
pub fn warn(message: &str) -> String {
    let mut output = String::from("[WARNING]: ");
    output.push_str(message);
    return output;
}
pub fn error(message: &str) -> String {
    let mut output = String::from("[ERROR]: ");
    output.push_str(message);
    return output;
}
