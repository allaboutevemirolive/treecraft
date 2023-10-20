use std::fs::DirEntry;
use std::io;

#[derive(Debug)]
pub enum Sort {
    /// `-ci` Sort file's name with case-Insensitive
    CaseInsensitive,

    /// `-cs` Sort file's name with case-Sensitive
    CaseSensitive,

    /// `-n` No sort
    None,

    // Size
    // Time
    // Version
    /// `-xt` sort based on file's extension
    Extension,
}

impl Default for Sort {
    fn default() -> Self {
        // The default sort in GNU ls is case-sensitive
        Sort::CaseSensitive
    }
}

#[allow(clippy::cognitive_complexity)]
#[cfg(any(unix, windows))]
#[inline(always)]
pub fn sort_entries(entries: &mut Vec<Result<DirEntry, io::Error>>, sort_type: &Sort) {
    match sort_type {
        Sort::CaseInsensitive => {
            entries.sort_unstable_by(|a, b| {
                let a_name = a.as_ref().unwrap().file_name();
                let b_name = b.as_ref().unwrap().file_name();
                a_name
                    .to_ascii_lowercase()
                    .cmp(&b_name.to_ascii_lowercase())
            });
        }
        // The default sort in GNU ls is case-sensitive
        Sort::CaseSensitive => {
            entries.sort_unstable_by_key(|entry| entry.as_ref().unwrap().file_name());
        }
        Sort::Extension => entries.sort_by(|a, b| {
            a.as_ref()
                .unwrap()
                .path()
                .extension()
                .cmp(&b.as_ref().unwrap().path().extension())
                .then(
                    a.as_ref()
                        .unwrap()
                        .path()
                        .file_stem()
                        .cmp(&b.as_ref().unwrap().path().file_stem()),
                )
        }),
        Sort::None => {}
    }
}
