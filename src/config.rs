//! This module contains the configuration options for the application
//!

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains the configuration options for controlling logging.
/// # Examples:
/// ```
/// use cli_utils::config::Loggin;
/// let log = Loggin::new();
/// let config = Loggin::new();
/// ```
///
/// Create a new `Loggin` struct with default values:
/// ```
/// use cli_utils::config::{Loggin, LogLevel, LogOutput};
/// let config = Loggin {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::Stdout,
/// };
/// ```
pub struct Loggin {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Loggin {
    pub fn new() -> Loggin {
        Loggin {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}