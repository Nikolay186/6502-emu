#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Verbose,
}

impl LogLevel {
    pub const ANSI_RESET: &'static str = "\x1b[0m";

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Verbose => "VERBOSE",
        }
    }

    pub fn ansi_color_code(&self) -> &'static str {
        match self {
            LogLevel::Error => "\x1b[31m",
            LogLevel::Warn => "\x1b[33m",
            LogLevel::Info => "\x1b[32m",
            LogLevel::Debug => "\x1b[95m",
            LogLevel::Verbose => "\x1b[96m",
        }
    }
}

impl From<u8> for LogLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => LogLevel::Error,
            1 => LogLevel::Warn,
            2 => LogLevel::Info,
            3 => LogLevel::Debug,
            4 => LogLevel::Verbose,
            _ => panic!("Invalid integer for log level: {value}"),
        }
    }
}

impl From<LogLevel> for u8 {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Info => 2,
            LogLevel::Debug => 3,
            LogLevel::Verbose => 4,
        }
    }
}
