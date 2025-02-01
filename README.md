# dates-str
## A small string date library written in Rust, for Rust

-----------------------------------
**NOTE**: DateStr subtraction and add work weird, since it's assumed all months have 30 days, so use them carefully
-----------------------------------

An easy crate for using and formatting dates. Works with ISO-8601 formatted dates by default, but you can format your dates with a custom formatter.

## Main
The main struct of this crate is the `DateStr` struct.
By default and as of now, it only accepts a date in ISO-8601 format. In the near future you will be able to pass any date formatted however you like, as long as you provide a `DateFormat`.


The `DateFormat` struct is responsible for parsing dates when they are in non-ISO fromat. Only works for output at the moment.

-----------------------------------
TODO:
- [ ] Date from custom format.
- [x] ~Check if month has correct day number. For example a date not beig the 31st of February~
- [x] ~Better implementations of Add and Sub traits for DateStr~ (It works, but with assumptions)
- [ ] Better README
- [ ] Implement unix epoch, maybe from std::time
------------------------------------

[docs.rs](https://docs.rs/dates-str/latest/dates_str) || [crates.io](https://crates.io/crates/dates-str)
