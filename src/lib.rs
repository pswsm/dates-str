//! dates_str - A date parser
//!
//! This crate, as it's name implies, it's not a "date & time" crate, but rather one to provide fast methods for handling datestrings:
//! from formatting to more advanced features (TBI) as addition, subtraction or checking if a date is valid, to name a few.
//!
//! There's a lot of assumptions in this crate, such as when adding or substracting months have 30 days.
//! Probably this coul be solved easily using a time crate, but I won't be checking that short-term.
//!
//! For full fledged date & time experiences, see:
//!  - [chrono](https://crates.io/crates/chrono)
//!  - [time](https://crates.io/crates/time)

#![deny(missing_docs)]

use std::fmt::Display;
use std::vec::Vec;

/// Tests
#[cfg(test)]
pub mod tests;

/// Error module
pub mod errors;

/// Traits and implementations module
pub mod impls;

/// Allowed formatter options
const FORMATTER_OPTIONS: [&str; 3] = ["YYYY", "MM", "DD"];

// #[allow(dead_code)]
// const EPOCH_DATE: &str = "1970-01-01";

/// Max number for february month
const MAX_DAY_FEBR: u8 = 29 as u8;

/// The date struct
///
/// Months and years are *1-indexed*, meaning they start at ONE (1). So January would be 1, as
/// written normally, and December is 12.
///
/// Called DateStr because it comes from a String
#[derive(Debug, PartialEq, Eq)]
pub struct DateStr {
    /// An unsigned 64-bit integer to hold the year
    year: Year,
    /// An unsigned 8-bit integer to hold the month
    month: Month,
    /// An unsigned 8-bit integer to hold the day
    day: Day,
}

impl DateStr {
    /// Creates a new DateStr from the given parts
    pub fn new(year: Year, month: Month, day: Day) -> Result<Self, errors::DateErrors> {
        if month.0 != 2 && day.0 > 29 {
            let err = errors::DateErrors::InvalidDay { day: day.0 };
            return Err(err);
        };
        Ok(Self { year, month, day })
    }
}

/// The `Day` struct. Holds a u8 because there's no 255 days.
///
/// On substractions it's value is casted to a i16 to allow for an ample range of negatives,
/// and then casted to u8 again on construction.
#[derive(Debug, Eq, PartialEq)]
pub struct Day(u8);

impl Day {
    /// Returns a new `Day` struct, or an [Err] of [`DateErrors`](crate::errors::DateErrors) if it exceeds 31.
    pub fn new(value: u8) -> Result<Self, errors::DateErrors> {
        if !(1..=31).contains(&value) {
            let err = errors::DateErrors::InvalidDay { day: value };
            return Err(err);
        };
        Ok(Self(value))
    }

    #[allow(dead_code)]
    fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::Add for Day {
    type Output = (Self, Month);
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.0 + rhs.0;
        let mut mo = 0;
        while sum > 30 {
            mo = mo + 1;
            sum = sum - 30;
        }
        (Self(sum), Month::new_unchecked(mo))
    }
}

impl std::ops::Sub for Day {
    type Output = (Self, Month);

    fn sub(self, rhs: Self) -> Self::Output {
        let mut sub = self.0 as i16 - rhs.0 as i16;
        let mut mos = 0;

        if sub > 0 {
            return (Self(sub as u8), Month::new_unchecked(mos));
        }

        while sub * -1 > 30 {
            mos = mos + 1;
            sub = sub + 30;
        }
        (Self(sub as u8), Month::new_unchecked(mos))
    }
}

/// The `Month` struct. Holds a u8 because there's just 12 months.
#[derive(Debug, Eq, PartialEq)]
pub struct Month(u8);

impl Month {
    /// Returns a new `Month` from a `u8`, or an error containing [`DateErrors`](crate::errors::DateErrors).
    pub fn new(value: u8) -> Result<Self, errors::DateErrors> {
        if !(1..=12).contains(&value) {
            return Err(errors::DateErrors::InvalidMonth { month: value });
        }
        Ok(Self(value))
    }

    fn new_unchecked(value: u8) -> Self {
        Self(value)
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::Add for Month {
    type Output = (Self, Year);
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.0 + rhs.0;
        let mut y2a: u64 = 0;
        while sum > 12 {
            y2a = y2a + 1;
            sum = sum - 12;
        }
        (Self(sum), Year::new(y2a))
    }
}

impl std::ops::Sub for Month {
    type Output = (Self, Year);
    fn sub(self, rhs: Self) -> Self::Output {
        let mut sub = self.0 as i16 - rhs.0 as i16;
        let mut yrs = 0;
        if sub > 0 {
            return (Self(sub as u8), Year::new(yrs));
        }
        sub = sub * (-1);
        while sub > 12 {
            yrs = yrs + 1;
            sub = sub - 12;
        }
        (Self(sub as u8), Year::new(yrs))
    }
}

/// The year struct. Holds a u64
#[derive(Debug, Eq, PartialEq)]
pub struct Year(u64);

impl Year {
    /// Creates a new `Year` from a number
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::Add for Year {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Year {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
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
            if !format
                .to_string()
                .split(separator)
                .any(|e| *e.to_uppercase() == *fmt_opt.to_string())
            {
                return Err(errors::DateErrors::FormatDateError);
            }
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
    /// I'd recommend using [crate::DateStr::try_from_iso_str] when unsure what the input string will be, since it
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
        let year: Year = Year::new(sep_date[0].parse::<u64>().unwrap_or_default());
        let month: Month = Month::new(sep_date[1].parse::<u8>().unwrap_or_default()).unwrap();
        if !(1..=12).contains(&month.0) {
            panic!("Month is out of bounds");
        }
        let day: Day = Day::new(sep_date[2].parse::<u8>().unwrap_or_default()).unwrap();
        let (month_ok, day_ok): (bool, bool) = DateStr::check_date_constraints(month.0, day.0);
        if !month_ok {
            panic!("Month {} is out of bounds", month);
        }
        if !day_ok {
            panic!("Day {} is out of bounds for month {}", day, month);
        }
        DateStr { year, month, day }
    }

    /// Checks if month and day are inside allowed range. Checks if day is within the months day
    /// too.
    ///
    /// Checks if month is within 1 and 12. Depending on month checks day is within that month's
    /// days. Returns a tuple with two bools: first is for the month, and second for the day.
    fn check_date_constraints(month: u8, day: u8) -> (bool, bool) {
        // TODO: improve this if .. else hell
        if !(1..=12).contains(&month) {
            return (false, false);
        }
        if month == 2 {
            if !(1..=MAX_DAY_FEBR).contains(&day) {
                (true, false)
            } else {
                (true, true)
            }
        } else if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
            if !(1..=31).contains(&day) {
                (true, false)
            } else {
                (true, true)
            }
        } else if [4, 6, 9, 11].contains(&month) {
            if !(1..31).contains(&day) {
                (true, false)
            } else {
                (true, true)
            }
        } else {
            (false, false)
        }
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
    pub fn try_from_iso_str<T: ToString>(string: T) -> Result<DateStr, errors::DateErrors> {
        let sep_date: Vec<String> = string
            .to_string()
            .split('-')
            .into_iter()
            .map(|split| split.to_string())
            .collect();
        let year: u64 = sep_date[0].parse::<u64>().unwrap_or_default();
        let month: u8 = sep_date[1].parse::<u8>().unwrap_or_default();
        if !(1..=12).contains(&month) {
            return Err(errors::DateErrors::InvalidMonth { month });
        };
        let day: u8 = sep_date[2].parse::<u8>().unwrap_or_default();
        if !(1..=31).contains(&day) {
            return Err(errors::DateErrors::InvalidDay { day });
        };
        Ok(DateStr {
            year: Year::new(year),
            month: Month::new(month).unwrap(),
            day: Day::new(day).unwrap(),
        })
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
    /// Use [crate::DateStr::try_format] for easy error handling
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
    /// To use errors see [crate::DateStr::try_format()]
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
