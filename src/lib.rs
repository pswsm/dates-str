//! The poorest yet easiest date-related crate to ever be published, probably
//! I'll be working on it to upgrade this library's capabilities
use std::fmt::Display;
use std::vec::Vec;
use std::marker::PhantomData;
use snafu::{
    Snafu,
    ensure
};

/// The only error struct this needs at the moment
/// Derives SNAFU to do error things
#[derive(Debug, Snafu)]
#[snafu(display("Invalid format: field {fld} not recognized"))]
pub struct FormatDateError {
    /// The field that was not recognized.
    /// 
    /// Let's say we pass this format: "yyyy-aa-dd".
    /// In this case, fld = "aa".
    fld: String
}

/// The date struct
/// Called DateStr because it comes from a String
pub struct DateStr<T>
where T: ToString
{
    /// An unsigned 64-bit integer to hold the year
    pub year: u64,
    /// An unsigned 8-bit integer to hold the month
    /// Does not check if it's in 1..12 or 0..11 range (yet)
    pub month: u8,
    /// An unsigned 8-bit integer to hold the day
    /// Does not check if it's in 1..31 or 0..30 range (yet)
    pub day: u8,
    /// I don't remember why I put this, but at this point I'm too afraid to remove it.
    _og_date: PhantomData<T>
}

impl<T> DateStr<T>
where T: ToString
{
    /// Parse a string to a DateStr struct
    /// Must be ISO format --> YYYY-MM-DD (2022-10-20)
    pub fn from_iso_str(string: T) -> DateStr<T>
    {
        let sep_date: Vec<String> = string.to_string().split('-').into_iter().map(|split| split.to_string() ).collect();
        let year: u64 = sep_date[0].parse::<u64>().unwrap_or_default();
        let month: u8  = sep_date[1].parse::<u8>().unwrap_or_default();
        let day: u8  = sep_date[2].parse::<u8>().unwrap_or_default();
        DateStr { year, month, day, _og_date: PhantomData }
    }
}

/// Display trait implementation for DateStr
/// Prints the date in ISO format (YYYY-MM-DD)
impl<T> Display for DateStr<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl<T> DateStr<T>
where T: ToString
{
    /// Format the date with a custom formatter. Will be optimised.
    /// Receives a String format, and a optional separator.
    /// 
    /// ```rust
    /// let a_date: DateStr = DateStr::from_iso_str("2022-12-29");
    /// println!("{}", a_date.format("dd-mm-yyyy", Some("/")));
    /// ```
    /// Above code will output `29/12/2022`
    ///
    /// Throws an error if a formatting field is not any of the following: `["yyyy", "mm", "dd"]`
    /// As said, there are no fancy features.
    pub fn format(&self, fmt: T, sep: Option<&str>) -> Result<String, FormatDateError> {
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
        let some_date: DateStr<_> = DateStr::from_iso_str("2022-11-16");
        assert_eq!(some_date.to_string(), "2022-11-16".to_owned());
    }

    #[test]
    fn fmt_date() {
        let some_date: DateStr<_> = DateStr::from_iso_str("2022-12-28");
        let fmt_date: String = some_date.format("DD-MM-YYYY", None).unwrap();
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn fmt_date_lowercase() {
        let some_date: DateStr<_> = DateStr::from_iso_str("2022-12-28");
        let fmt_date: String = some_date.format("dd-mm-yyyy", None).unwrap();
        assert_eq!(fmt_date.to_string(), "28-12-2022".to_owned());
    }

    #[test]
    fn fmt_date_error() {
        let some_date: DateStr<_> = DateStr::from_iso_str("2022-12-28");
        let fmt_date: Result<String, FormatDateError> = some_date.format("DD-MM-YYAY", None);
        assert!(fmt_date.is_err());
    }
}
