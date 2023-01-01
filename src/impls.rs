#![deny(missing_docs)]

use crate::DateStr;

/// Trait for easy DateStr making
///
/// Blank implementation
pub trait ToDateStr {
    /// This function creates a DateStr in a to_string() fashion
    fn to_datestr(&self) -> DateStr;
}

/// Implementation of ToDateStr for String
impl ToDateStr for String {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }
}

/// Implementation of ToDateStr for &str
impl ToDateStr for str {
    fn to_datestr(&self) -> DateStr {
        DateStr::from_iso_str(self)
    }
}

/// Implementation of ToDateStr for DateStr
impl ToDateStr for DateStr {
    fn to_datestr(&self) -> DateStr {
        self.clone()
    }
}
