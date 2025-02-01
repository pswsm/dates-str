#[cfg(test)]
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
    assert_eq!(date, DateStr {
        year: Year::new(2023),
        month: Month::new_unchecked(1),
        day: Day::new_unchecked(2)
    });
}

#[test]
#[should_panic]
fn check_feb_day_oobp() {
    let _date: DateStr = "2023-02-30".to_datestr();
}

#[test]
#[should_panic]
fn check_31_day_oobp() {
    let _date: DateStr = "2023-04-31".to_datestr();
}

#[test]
#[should_panic]
fn check_32_day_oobp() {
    let _date: DateStr = "2023-01-32".to_datestr();
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

#[test]
fn add_one_month() {
    let month = Month::new(2).unwrap();
    let month2 = Month::new(2).unwrap();
    assert_eq!(month + month2, (Month::new(4).unwrap(), Year::new(0)))
}
