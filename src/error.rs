use std::fmt;

/// Represents the possible ways how creating an IntDate may fail.
#[derive(Debug, Clone, Copy)]
pub enum IntDateError {
    /// The provided way was invalid (possibly for the current month).
    InvalidDay,
    /// The provided month was invalid (not 1 to 12).
    InvalidMonth,
    /// Parsing from a string has failed.
    ParsingFailed,
}

impl std::error::Error for IntDateError {}

impl fmt::Display for IntDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IntDateError::*;
        let message = match self {
            InvalidDay => "Invalid day.",
            InvalidMonth => "Invalid month.",
            ParsingFailed => "Parsing failed.",
        };
        write!(f, "{}", message)
    }
}
