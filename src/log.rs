use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogSeverity {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

impl std::str::FromStr for LogSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UVM_INFO" => Ok(LogSeverity::INFO),
            "UVM_WARNING" => Ok(LogSeverity::WARNING),
            "UVM_ERROR" => Ok(LogSeverity::ERROR),
            "UVM_FATAL" => Ok(LogSeverity::FATAL),
            _ => Err(format!("'{}' is not a valid severity", s)),
        }
    }
}

impl fmt::Display for LogSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogSeverity::INFO => write!(f, "UVM_INFO"),
            LogSeverity::WARNING => write!(f, "UVM_WARNING"),
            LogSeverity::ERROR => write!(f, "UVM_ERROR"),
            LogSeverity::FATAL => write!(f, "UVM_FATAL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogLine {
    pub severity: LogSeverity,
    pub file: PathBuf,
    pub line: u32,
    pub time: u64,
    pub component: String,
    pub id: String,
    pub message: String,
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}({}) @ {}: {} [{}] {}",
            self.severity, self.file.to_string_lossy(), self.line, self.time, self.component, self.id, self.message
        )
    }
}


#[derive(Debug, Clone)]
pub struct Log {
    logs: Vec<LogLine>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            logs: Vec::new()
        }
    }

    pub fn push(&mut self, line: LogLine) {
        self.logs.push(line);
    }
}

impl IntoIterator for Log {
    type Item = LogLine;
    type IntoIter = std::vec::IntoIter<LogLine>;

    fn into_iter(self) -> Self::IntoIter {
        self.logs.into_iter()
    }
}