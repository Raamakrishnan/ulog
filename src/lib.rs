pub mod log;
pub mod parser;

/// Type-erased errors
pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

