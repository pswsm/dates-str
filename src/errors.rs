#![deny(missing_docs)]
use snafu::Snafu;

#[derive(Debug, Snafu)]
/// Errors on date boundaries
///
/// The errors given when a date is out of bounds, for example a 13th month or the 41st day of a
/// month.
///
/// Exmple:
/// ```rust
/// # use dates_str::DateStr;
/// // TODO: Finish this example
/// ```
pub enum DateErrors {
    /// Enum variant when day is out of bounds
    #[snafu(
        display("Day must be 0 <= day >= 30. It was {day}"),
        visibility(pub(crate)),
        context(suffix(Ctx))
    )]
    InvalidDay {
        /// They "day" that provoked the error.
        day: u8,
    },
    /// Enum variant when month is out of bounds
    #[snafu(
        display("Month must be 0 <= month >= 11. It was {month}"),
        visibility(pub(crate)),
        context(suffix(Ctx))
    )]
    InvalidMonth {
        /// The "month" that provoked the error
        month: u8,
    },
    /// Enum variant when a formatter field is not resolved
    #[snafu(display("Format not recognized"), visibility(pub(crate)))]
    FormatDateError,
}
