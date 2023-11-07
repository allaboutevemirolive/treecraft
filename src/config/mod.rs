pub mod all;
pub mod default;

use std::ffi::OsString;
use std::fmt;
use std::fs::DirEntry;
use std::path::PathBuf;

use self::all::ConfigAll;
use self::default::ConfigDefault;

#[derive(Debug, Default)]
pub enum Config {
    All,
    #[default]
    Default,
}

/// This wrapper enables the return of different types.
/// 1. `Default`: Collect minimal metada and improve.
/// 2. `All`: Collect all metadata.
///
/// ### INFO
///
/// Enum variants in Rust should have the same type,
/// thus we need enum wrapper.
#[derive(Debug)]
pub enum ConfigInfo {
    /// Collect all metada
    All(ConfigAll),
    /// Collect default metada
    Default(ConfigDefault),
}

// FIXME
impl ConfigInfo {
    #[rustfmt::skip]
    pub fn new(
        entry: &DirEntry,
        depth: &i32,
        config: &Config,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match config {
            Config::All => Ok(
                ConfigInfo::All(ConfigAll::new(entry, depth)?)
            ),
            Config::Default => Ok(
                ConfigInfo::Default(ConfigDefault::new(entry, depth)?)
            ),
        }
    }
}

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
