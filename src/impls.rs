#![deny(missing_docs)]

use crate::DateStr;
use std::ops::{Add, Sub};

/// Trait for easy DateStr making
///
/// Blank implementation
pub trait ToDateStr {
    /// This function creates a [crate::DateStr] in a to_string() fashion
    fn to_datestr(&self) -> DateStr;

    /// Try to convert to DateStr using [crate::DateStr::try_from_iso_str] function, which returns a
    /// Result enum.
    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors>;
}

/// Implementation of ToDateStr for String
impl ToDateStr for String {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }

    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors> {
        DateStr::try_from_iso_str(self)
    }
}

/// Implementation of ToDateStr for &str
impl ToDateStr for str {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }

    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors> {
        DateStr::try_from_iso_str(self)
    }
}

/// Implementation of ToDateStr for DateStr
impl ToDateStr for DateStr {
    fn to_datestr(&self) -> DateStr {
        self.clone()
    }

    fn try_to_datestr(&self) -> Result<DateStr, crate::errors::DateErrors> {
        DateStr::try_from_iso_str(self)
    }
}

impl From<String> for DateStr {
    fn from(value: String) -> Self {
        let split: Vec<String> = value.split('-').map(|s| s.to_string()).collect();
        DateStr {
            year: split[0].parse::<u64>().unwrap_or(2020),
            month: split[1].parse::<u8>().unwrap_or(0),
            day: split[2].parse::<u8>().unwrap_or(0),
        }
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
        let (day, add_mo): (u8, u8) = {
            let mut day = self.day + rhs.day;
            let mut add_mo: u8 = 0;
            while day > 31 {
                day -= 31;
                add_mo += 1;
            }
            (day, add_mo)
        };
        let (month, add_year): (u8, u64) = {
            let mut months: u8 = self.month + rhs.month + add_mo;
            let mut add_year: u64 = 0;
            while months > 12 {
                add_year += 1;
                months -= 12;
            }
            (months, add_year)
        };
        let year = self.year + rhs.year + add_year;

        DateStr { day, month, year }
    }
}

impl Sub for DateStr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (day, sub_mo): (u8, u8) = {
            let mut day = self.day as i8 - rhs.day as i8;
            let mut sub_mo: u8 = 0;
            while day < 1 {
                if day == 0 {
                    day += 1;
                } else {
                    day += 31;
                }
                sub_mo += 1;
            }
            (day as u8, sub_mo)
        };
        let (month, sub_year): (u8, u64) = {
            let mut months: i8 = self.month as i8 + rhs.month as i8 + sub_mo as i8;
            let mut sub_year: u64 = 0;
            while months < 1 {
                sub_year += 1;
                if months == 0 {
                    months += 1;
                } else {
                    months += 12;
                }
            }
            (months as u8, sub_year)
        };
        let year = self.year - rhs.year - sub_year;

        DateStr { day, month, year }
    }
}
