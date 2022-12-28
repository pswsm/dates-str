use std::fmt::Display;
use std::vec::Vec;
use std::marker::PhantomData;
use snafu::{
    Snafu,
    ensure
};

#[derive(Debug, Snafu)]
#[snafu(display("Invalid format: field {fld} not reconized"))]
pub struct FormatDateError {
    fld: String
}

pub struct DateStr<T>
where T: ToString
{
    pub year: u32,
    pub month: u8,
    pub day: u8,
    _og_date: PhantomData<T>
}

impl<T> DateStr<T>
where T: ToString
{
    pub fn from_iso_str(string: T) -> DateStr<T>
    {
        let sep_date: Vec<String> = string.to_string().split('-').into_iter().map(|split| split.to_string() ).collect();
        let year: u32 = sep_date[0].parse::<u32>().unwrap_or_default();
        let month: u8  = sep_date[1].parse::<u8>().unwrap_or_default();
        let day: u8  = sep_date[2].parse::<u8>().unwrap_or_default();
        DateStr { year, month, day, _og_date: PhantomData }
    }
}

/// Displays in ISO format (YYYY-MM-DD)
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
