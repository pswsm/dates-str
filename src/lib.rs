//! The easiest date-related crate to ever be published, probably
//!
//! This crate, as it's name implies, it's not a "date & time" crate per se, it does not provide
//! such functionalities. What this crate provides are fast methods for handling "datestrings", 
//! from formatting to more advanced features (to be implemented) as sum, subtraction and checking if a date is valid, to name a few.
//!
//! For a full fledged date & time experiences, see:
//!  - [chrono](https://crates.io/crates/chrono)
//!  - [time](https://crates.io/crates/time)

#![deny(missing_docs)]

use std::fmt::Display;
use std::vec::Vec;
use snafu::{
    Snafu,
    ensure
};

/// Error given when a formatter field is not resolved
///
/// Derives SNAFU to do error things
#[derive(Debug, Snafu)]
#[snafu(display("Invalid format: field {fld} not recognized"))]
pub struct FormatDateError {
    fld: String
}

/// The date struct
///
/// Called DateStr because it comes from a String
#[derive(PartialEq)]
pub struct DateStr
{
    /// An unsigned 64-bit integer to hold the year
    pub year: u64,
    /// An unsigned 8-bit integer to hold the month
    /// Does not check if it's in 1..12 or 0..11 range (yet)
    pub month: u8,
    /// An unsigned 8-bit integer to hold the day
    /// Does not check if it's in 1..31 or 0..30 range (yet)
    pub day: u8,
}

impl DateStr
{
    /// Parse a string to a DateStr struct
    ///
    /// Parses a string (or any type implementing the [ToString] trait) to a DateStr struct.
    /// The given date must be in ISO-8601 format, that is: YYYY-MM-DD.
    ///
    /// # Examples
    /// ```rust
    /// # use dates_str::DateStr;
    /// let date_string: String = String::from("2022-12-31");
    /// let new_date_from_string: DateStr<_> = DateStr::from_iso_str(date_string);
    /// let new_date_from_str: DateStr<_> = DateStr::from_iso_str("2022-12-31");
    /// assert_eq!(new_date_from_str, new_date_from_string);
    /// ```
    pub fn from_iso_str<T: ToString>(string: T) -> DateStr
    {
        let sep_date: Vec<String> = string.to_string().split('-').into_iter().map(|split| split.to_string() ).collect();
        let year: u64 = sep_date[0].parse::<u64>().unwrap_or_default();
        let month: u8  = sep_date[1].parse::<u8>().unwrap_or_default();
        let day: u8  = sep_date[2].parse::<u8>().unwrap_or_default();
        DateStr { year, month, day }
    }
}

/// Display trait implementation for DateStr
///
/// Prints the date in ISO format (YYYY-MM-DD)
impl Display for DateStr
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl DateStr
{
    /// Format the date with a custom formatter. Will be optimised.
    /// Receives a String format, and a optional separator.
    /// 
    /// ```rust
    /// # use dates_str::DateStr;
    /// let a_date: DateStr<_> = DateStr::from_iso_str("2022-12-29");
    /// let formatted_date: String = a_date.format("dd-mm-yyyy", Some("/")).unwrap();
    /// println!("{}", formatted_date);
    /// ```
    /// Above code will output `29/12/2022`
    ///
    /// Throws an error if a formatting field is not any of the following: `["yyyy", "mm", "dd"]`
    /// As said, there are no fancy features.
    pub fn format<T: ToString>(&self, fmt: T, sep: Option<&str>) -> Result<String, FormatDateError> {
        let allowed_formats: Vec<&str> = vec!["YYYY", "MM", "DD"];
        let binding: String = fmt.to_string().to_uppercase();
        let format: Vec<&str> = binding.splitn(3, |sep| sep == '-' || sep == '/').collect();
        for formatter in format.iter() {
            ensure!(allowed_formats.iter().any(|e| *e.to_string() == *formatter.to_string()), FormatDateSnafu { fld: formatter.to_string() })
        }
        let formatted: Vec<String> = format.into_iter().map(|f| 
            match f {
                "YYYY" => self.year.clone().to_string(),
                "MM" => self.month.clone().to_string(),
                "DD" => self.day.clone().to_string(),
                &_ => unreachable!()
            }).collect();
        if let Some(separator) = sep {
            return Ok(formatted.join(separator))
        }
        Ok(formatted.join("-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_str() {
        let some_date: DateStr = DateStr::from_iso_str("2022-11-16");
        assert_eq!(some_date.to_string(), "2022-11-16".to_owned());
    }

    #[test]
    fn fmt_date() {
        let some_date: DateStr = DateStr::from_iso_str("2022-12-28");
        let fmt_date: String = some_date.format("DD-MM-YYYY", None).unwrap();
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn fmt_date_lowercase() {
        let some_date: DateStr = DateStr::from_iso_str("2022-12-28");
        let fmt_date: String = some_date.format("dd-mm-yyyy", None).unwrap();
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn fmt_date_error() {
        let some_date: DateStr = DateStr::from_iso_str("2022-12-28");
        let fmt_date: Result<String, FormatDateError> = some_date.format("DD-MM-YYAY", None);
        assert!(fmt_date.is_err());
    }
}
