use std::borrow::Cow;
use std::ffi::OsString;
use std::fmt;

pub mod all;
pub mod default;

pub struct DisplayFormatted<'a, T> {
    pub content: &'a T,
    pub format_fn: fn(&T, &mut fmt::Formatter) -> fmt::Result,
}

impl<'a, T> fmt::Display for DisplayFormatted<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.format_fn)(self.content, f)
    }
}

// // Example format functions
// fn format_default_str<'a>(s: &'a str, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", s)
// }

// fn format_bright_green_str<'a>(s: &'a str, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "\x1b[1;32m{}\x1b[0m", s)
// }

// fn format_name_path<'a>(
//     (name, path): &'a (OsString, PathBuf),
//     f: &mut fmt::Formatter,
// ) -> fmt::Result {
//     write!(f, "{} ({})", name.to_string_lossy(), path.to_string_lossy())
// }

pub fn format_cow_str<'a>(s: &'a Cow<'a, str>, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", s)
}

pub fn format_bright_green_cow_str<'a>(s: &'a Cow<'a, str>, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\x1b[1;32m{}\x1b[0m", s)
}

pub fn format_os_string(s: &OsString, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", s.to_string_lossy())
}

pub fn format_bright_green_os_string(s: &OsString, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\x1b[1;32m{}\x1b[0m", s.to_string_lossy())
}
