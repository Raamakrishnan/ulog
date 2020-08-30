use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Severity {
    INFO,
    WARNING,
    ERROR,
    FATAL,
}

impl std::str::FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UVM_INFO" | "INFO" | "info" => Ok(Severity::INFO),
            "UVM_WARNING" | "WARNING" | "WARN" | "warn" | "warning" => Ok(Severity::WARNING),
            "UVM_ERROR" | "ERROR" | "error" => Ok(Severity::ERROR),
            "UVM_FATAL" | "FATAL" | "fatal" => Ok(Severity::FATAL),
            _ => Err(format!("'{}' is not a valid severity", s)),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Severity::INFO => write!(f, "UVM_INFO"),
            Severity::WARNING => write!(f, "UVM_WARNING"),
            Severity::ERROR => write!(f, "UVM_ERROR"),
            Severity::FATAL => write!(f, "UVM_FATAL"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Picosecond,
    Femtosecond,
}

impl std::str::FromStr for TimeUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(TimeUnit::Second),
            "ms" => Ok(TimeUnit::Millisecond),
            "us" => Ok(TimeUnit::Microsecond),
            "ns" => Ok(TimeUnit::Nanosecond),
            "ps" => Ok(TimeUnit::Picosecond),
            "fs" => Ok(TimeUnit::Femtosecond),
            _ => Err(format!("'{}' is not a valid time unit", s)),
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimeUnit::Second => write!(f, "s"),
            TimeUnit::Millisecond => write!(f, "ms"),
            TimeUnit::Microsecond => write!(f, "us"),
            TimeUnit::Nanosecond => write!(f, "ns"),
            TimeUnit::Picosecond => write!(f, "ps"),
            TimeUnit::Femtosecond => write!(f, "fs"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub severity: Severity,
    pub file: PathBuf,
    pub line: u32,
    pub time: u64,
    pub time_unit: Option<TimeUnit>,
    pub component: String,
    pub id: String,
    pub message: String,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.time_unit {
            Some(unit) => write!(
                f,
                "{} {}({}) @ {}{}: {} [{}] {}",
                self.severity,
                self.file.to_string_lossy(),
                self.line,
                self.time,
                unit.to_string(),
                self.component,
                self.id,
                self.message
            ),
            None => write!(
                f,
                "{} {}({}) @ {}: {} [{}] {}",
                self.severity,
                self.file.to_string_lossy(),
                self.line,
                self.time,
                self.component,
                self.id,
                self.message
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    logs: Vec<Line>,
}

impl Log {
    pub fn new() -> Log {
        Log { logs: Vec::new() }
    }

    pub fn push(&mut self, line: Line) {
        self.logs.push(line);
    }
}

impl IntoIterator for Log {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.logs.into_iter()
    }
}
