use chrono::Local;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

pub struct Logger {
    prefix: String,
}

impl Logger {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{timestamp}] [{level}] [{}] {message}", self.prefix);
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }
    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }
    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }
    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}
