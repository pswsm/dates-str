//! dates_str - A date parser
//!
//! This crate, as it's name implies, it's not a "date & time" crate per se, instead provides fast methods for handling datestrings,
//! from formatting to more advanced features (to be implemented) as addition, subtraction or checking if a date is valid, to name a few.
//!
//! For a full fledged date & time experiences, see:
//!  - [chrono](https://crates.io/crates/chrono)
//!  - [time](https://crates.io/crates/time)

#![deny(missing_docs)]

use snafu::ensure;
use std::fmt::Display;
use std::vec::Vec;

/// Error module
pub mod errors;

/// Traits and implementations module
pub mod impls;

const FORMATTER_OPTIONS: [&str; 3] = ["YYYY", "MM", "DD"];
const EPOCH_DATE: &str = "1970-1-1";

/// The date struct
///
/// Months and years are *1-indexed*, meaning they start at ONE (1). So January would be 1, as
/// written normally, and December is 12, unlike JS where months are 0-indexed.
///
/// Called DateStr because it comes from a String
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DateStr {
    /// An unsigned 64-bit integer to hold the year
    pub year: u64,
    /// An unsigned 8-bit integer to hold the month
    pub month: u8,
    /// An unsigned 8-bit integer to hold the day
    pub day: u8,
}

/// The format a [DateStr] will be printed
#[derive(Debug)]
pub struct DateFormat {
    /// The format to be used
    pub formatter: String,
}

impl DateFormat {
    /// Creates a DateFormat from String or a &str
    ///
    /// This method will try to create a [DateFormat] from any type that implements the ToString
    /// type, although is mainly oriented to String and string slices.
    ///
    /// # Example:
    /// ```rust
    /// # use dates_str::DateFormat;
    /// let format: DateFormat = DateFormat::from_string("YYYY-MM-DD", None).unwrap();
    /// assert_eq!(format.formatter, "YYYY-MM-DD");
    /// ```
    /// Above code will create a new DateFormat object. If none is passed as separator, it defaults
    /// to a dash ('-').
    ///
    /// # Example returning error:
    /// ```rust
    /// # use dates_str::{DateStr, DateFormat, errors::DateErrors};
    /// let format: Result<DateFormat, DateErrors> = DateFormat::from_string("2020_10_20", Some('/'));
    /// assert!(format.is_err());
    /// ```
    ///
    /// When the separator is not explicitly specified, it will give an error if it's not a dash.
    pub fn from_string<T: ToString>(
        format: T,
        separator: Option<char>,
    ) -> Result<DateFormat, errors::DateErrors> {
        let separator: char = separator.unwrap_or('-');
        for fmt_opt in FORMATTER_OPTIONS {
            ensure!(
                format
                    .to_string()
                    .split(separator)
                    .any(|e| *e.to_uppercase() == *fmt_opt.to_string()),
                errors::FormatDateSnafu
            )
        }
        Ok(DateFormat {
            formatter: format.to_string().to_uppercase(),
        })
    }
}

impl DateStr {
    /// Parse a string to a DateStr struct
    ///
    /// Parses a string (or any type implementing the [ToString] trait) to a DateStr struct.
    ///
    /// The given date must be in ISO-8601 format, that is: YYYY-MM-DD.
    ///
    /// I'd recommend using [try_from_iso_str] when unsure what the input string will be, since it
    /// returns a Result with understandable errors.
    ///
    /// # Examples
    /// ```rust
    /// # use dates_str::DateStr;
    /// let date_string: String = String::from("2022-12-31");
    /// let new_date_from_string: DateStr = DateStr::from_iso_str(date_string);
    /// let new_date_from_str: DateStr = DateStr::from_iso_str("2022-12-31");
    /// assert_eq!(new_date_from_str, new_date_from_string);
    /// ```
    pub fn from_iso_str<T: ToString>(string: T) -> DateStr {
        let sep_date: Vec<String> = string
            .to_string()
            .split('-')
            .into_iter()
            .map(|split| split.to_string())
            .collect();
        let year: u64 = sep_date[0].parse::<u64>().unwrap_or_default();
        let month: u8 = sep_date[1].parse::<u8>().unwrap_or_default();
        if !(0..=11).contains(&month) {
            panic!("Month is out of bounds");
        }
        let day: u8 = sep_date[2].parse::<u8>().unwrap_or_default();
        if !(0..=31).contains(&day) {
            panic!("Day is out of bounds");
        }
        DateStr { year, month, day }
    }
    /// Parse a string to a DateStr struct
    ///
    /// Parses a string (or any type implementing the [ToString] trait) to a DateStr struct. This
    /// function returns a Result enum.
    ///
    /// The given date must be in ISO-8601 format, that is: YYYY-MM-DD.
    ///
    /// # Examples
    /// ```rust
    /// # use dates_str::DateStr;
    /// # use dates_str::errors;
    /// let date_string: String = String::from("2022-12-31");
    /// let date_from_string: Result<DateStr, errors::DateErrors> = DateStr::try_from_iso_str(date_string);
    /// assert!(date_from_string.is_ok());;
    /// ```
    ///
    /// # Errors
    /// Since it checks for month first, it will return a DateErrors::InvalidMonth even if the day
    /// is wrong too, in wich it would return a DateErrors::InvalidDay.
    ///
    /// Both of this variants have a day and month field respectively, and are built with SNAFU so
    /// they have many useful functions to print or do stuff.
    pub fn try_from_iso_str<T: ToString>(string: T) -> Result<DateStr, errors::DateErrors> {
        let sep_date: Vec<String> = string
            .to_string()
            .split('-')
            .into_iter()
            .map(|split| split.to_string())
            .collect();
        let year: u64 = sep_date[0].parse::<u64>().unwrap_or_default();
        let month: u8 = sep_date[1].parse::<u8>().unwrap_or_default();
        ensure!((1..=12).contains(&month), errors::InvalidMonthCtx { month });
        let day: u8 = sep_date[2].parse::<u8>().unwrap_or_default();
        ensure!((1..=31).contains(&day), errors::InvalidDayCtx { day });
        Ok(DateStr { year, month, day })
    }
}

/// Display trait implementation for DateStr
///
/// Prints the date in ISO-8601 format (YYYY-MM-DD)
impl Display for DateStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl DateStr {
    /// Format the date with a [DateFormat]
    ///
    /// Pass a [DateFormat]. Will output a String with the date formatted how you wanted.
    ///
    /// Use [try_format] for easy error handling
    ///
    /// # Example
    /// ```rust
    /// # use dates_str::{DateStr, DateFormat};
    /// let a_date: DateStr = DateStr::from_iso_str("2022-12-29");
    /// let a_fmtr: DateFormat = DateFormat::from_string("dd_mm_yyyy", Some('_')).unwrap();
    /// let formatted_date: String = a_date.format(a_fmtr);
    /// println!("{}", formatted_date);
    /// ```
    /// Above code will output 29-12-2022.
    ///
    /// # Panics
    /// This function will panic when an invalid [DateFormat] is passed.
    /// 
    /// To use errors see [try_format()]
    pub fn format(&self, fmt: DateFormat) -> String {
        let self_fmtd: String = fmt
            .formatter
            .replace("YYYY", &self.year.to_string())
            .replace("MM", &self.month.to_string())
            .replace("DD", &self.day.to_string());
        self_fmtd
    }

    /// Try to format the date with a custom formatter
    ///
    /// Safe function using the Result enum.
    /// Receives a [DateFormat] struct.
    ///
    /// # Example:
    /// ```rust
    /// # use dates_str::{DateStr, DateFormat};
    /// let a_date: DateStr = DateStr::from_iso_str("2022-12-29");
    /// let some_formatter: DateFormat = DateFormat::from_string("dd-mm-yyyy", None).unwrap();
    /// let formatted_date: String = a_date.try_format(some_formatter).unwrap();
    /// println!("{}", formatted_date);
    /// ```
    /// Will output 29-12-2022
    pub fn try_format(&self, fmt: DateFormat) -> Result<String, errors::DateErrors> {
        let self_fmtd: String = fmt
            .formatter
            .replace("YYYY", &self.year.to_string())
            .replace("MM", &self.month.to_string())
            .replace("DD", &self.day.to_string());
        Ok(self_fmtd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::DateErrors;
    use crate::impls::*;

    #[test]
    fn test_iso_str() {
        let some_date: DateStr = DateStr::from_iso_str("2022-11-16");
        assert_eq!(some_date.to_string(), "2022-11-16".to_owned());
    }

    #[test]
    fn date_fmt() {
        let some_date: DateStr = DateStr::from_iso_str("2022-12-28");
        let some_formatter: DateFormat = DateFormat::from_string("dd-mm-yyyy", None).unwrap();
        let fmt_date: String = some_date.format(some_formatter);
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn date_lowercase_fmt() {
        let some_date: DateStr = DateStr::from_iso_str("2022-12-28");
        let some_formatter: DateFormat = DateFormat::from_string("dd-mm-yyyy", None).unwrap();
        let fmt_date: String = some_date.try_format(some_formatter).unwrap();
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn formatter_error() {
        let some_formatter: Result<DateFormat, DateErrors> =
            DateFormat::from_string("dd-mm-yyay", None);
        assert!(some_formatter.is_err());
    }

    #[test]
    fn trait_to_date() {
        let date: DateStr = "2023-01-02".to_datestr();
        assert_eq!(date.to_string(), "2023-01-02".to_string());
    }

    #[test]
    #[should_panic]
    fn check_day_oobp() {
        let _date: DateStr = "2023-12-32".to_datestr();
    }

    #[test]
    #[should_panic]
    fn check_month_oobp() {
        let _date: DateStr = "2023-55-02".to_datestr();
    }

    #[test]
    fn check_day_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023-12-32".try_to_datestr();
        assert!(date.is_err());
    }

    #[test]
    fn check_month_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023-55-02".try_to_datestr();
        assert!(date.is_err());
    }

    #[test]
    fn check_negative_day_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023-12--3".try_to_datestr();
        assert!(date.is_err());
    }

    #[test]
    fn check_negative_month_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023--11-02".try_to_datestr();
        assert!(date.is_err());
    }

    #[test]
    fn check_zero_day_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023-12-0".try_to_datestr();
        assert!(date.is_err());
    }

    #[test]
    fn check_zero_month_oob() {
        let date: Result<DateStr, errors::DateErrors> = "2023-0-02".try_to_datestr();
        assert!(date.is_err());
    }
}
