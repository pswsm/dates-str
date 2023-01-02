#![deny(missing_docs)]

use crate::DateStr;

/// Trait for easy DateStr making
///
/// Blank implementation
pub trait ToDateStr {
    /// This function creates a DateStr in a to_string() fashion
    fn to_datestr(&self) -> DateStr;

    /// Try to convert to DateStr using [DatesStr::try_from_iso_str] function, which returns a
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
