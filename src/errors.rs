use snafu::Snafu;

/// Error given when a formatter field is not resolved
///
/// Derives SNAFU to do error things
#[derive(Debug, Snafu)]
#[snafu(display("Format not recognized"), visibility(pub(crate)))]
pub struct FormatDateError {}

impl FormatDateError {
    fn new() -> FormatDateError {
        FormatDateError {}
    }
}

impl Default for FormatDateError {
    fn default() -> Self {
        Self::new()
    }
}
