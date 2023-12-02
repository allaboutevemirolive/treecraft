use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;

pub mod all;
pub mod default;

pub struct DisplayOsString<'a>(pub &'a OsString);

impl<'a> fmt::Display for DisplayOsString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

/// Meant for output in a terminal with ANSI support
pub struct DisplayBrightGreen<'a>(pub &'a OsString);

impl<'a> fmt::Display for DisplayBrightGreen<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Apply bright green color formatting to the string,
        // insert the OsString, and reset the color to default
        write!(f, "\x1b[1;32m{}\x1b[0m", self.0.to_string_lossy(),)
    }
}

pub struct DisplayNamePath<'a>(pub &'a OsString, pub &'a PathBuf);

// DisplayNamePath(&info.name, &info.path)
impl<'a> fmt::Display for DisplayNamePath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.0.to_string_lossy(),
            self.1.to_string_lossy()
        )
    }
}
