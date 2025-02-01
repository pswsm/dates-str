#![deny(missing_docs)]

use crate::{DateStr, errors::DateErrors};
use std::ops::{Add, Sub};

/// Trait for easy DateStr making
///
/// Blank implementation
pub trait Into<DateStr> {
    /// This function creates a [crate::DateStr] in a to_string() fashion
    fn to_datestr(&self) -> DateStr;

    /// Try to convert to DateStr using [crate::DateStr::try_from_iso_str] function, which returns a
    /// Result enum.
    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors>;
}

/// Implementation of ToDateStr for String
impl Into<DateStr> for String {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }

    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors> {
        DateStr::try_from_iso_str(self)
    }
}

/// Implementation of ToDateStr for &str
impl Into<DateStr> for str {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }

    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors> {
        DateStr::try_from_iso_str(self)
    }
}

impl TryFrom<String> for DateStr {
    type Error = DateErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<String> = value.split('-').map(|s| s.to_string()).collect();
        let year = match split[0].parse::<u64>() {
            Ok(y) => crate::Year::new(y),
            Err(_e) => return Err(DateErrors::InvalidParsing(value)),
        };
        let month = match split[1].parse::<u8>() {
            Ok(y) => crate::Month::new(y)?,
            Err(_e) => return Err(DateErrors::InvalidParsing(value)),
        };
        let day = match split[2].parse::<u8>() {
            Ok(y) => crate::Day::new(y)?,
            Err(_e) => return Err(DateErrors::InvalidParsing(value)),
        };
        Ok(DateStr { year, month, day })
    }
}

impl From<DateStr> for String {
    fn from(value: DateStr) -> Self {
        value.to_string()
    }
}

impl Add for DateStr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (day, months_from_day) = self.day + rhs.day;
        let (months, years) = self.month + rhs.month;
        let (month, more_years) = months + months_from_day;
        let year = self.year + rhs.year + years + more_years;
        DateStr { day, month, year }
    }
}

impl Sub for DateStr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (day, months_from_day) = self.day - rhs.day;
        let (months, years) = self.month - rhs.month;
        let (month, more_years) = months - months_from_day;
        let year = self.year - rhs.year - years - more_years;
        DateStr { day, month, year }
    }
}
