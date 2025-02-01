use std::fmt::Display;

#[derive(Debug)]
/// Errors on date boundaries
///
/// The errors given when a date is out of bounds, for example a 13th month or the 41st day of a
/// month.
pub enum DateErrors {
    /// Enum variant when day is out of bounds
    InvalidDay {
        /// They "day" that provoked the error.
        day: u8,
    },
    /// Enum variant when month is out of bounds
    InvalidMonth {
        /// The "month" that provoked the error
        month: u8,
    },
    /// Enum variant when a formatter field is not resolved
    FormatDateError,
    /// Invalid year variant.
    InvalidYear(u64),

    /// Error to return when triying to parse something that cannot be respresented as a number
    InvalidParsing(String),
}

impl Display for DateErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDay { day } => write!(f, "Invalid Day: provided {}", day),
            Self::InvalidMonth { month } => write!(f, "Invalid Month: provided {}", month),
            Self::FormatDateError => write!(f, "Format not recognized"),
            Self::InvalidYear(year) => write!(f, "Invalif year provided: {}", year),
            Self::InvalidParsing(s) => write!(f, "Cannot parse {}: not a number...", s),
        }
    }
}

impl std::error::Error for DateErrors {}
