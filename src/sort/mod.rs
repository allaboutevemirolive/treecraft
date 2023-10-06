use std::fs::DirEntry;
use std::io;

#[derive(Debug)]
pub enum SortType {
    /// `-st-lc` Sort by file's name with case insensitive
    ByLowerCaseFileName,

    /// `-st-fn` Sort by file's name only
    ByFileName,

    /// `-st-no` No sort
    NoSort,
}

impl Default for SortType {
    fn default() -> Self {
        SortType::ByLowerCaseFileName
    }
}

pub fn sort_entries(entries: &mut Vec<Result<DirEntry, io::Error>>, sort_type: &SortType) {
    match sort_type {
        SortType::ByLowerCaseFileName => {
            entries.sort_unstable_by(|a, b| {
                let a_name = a.as_ref().unwrap().file_name();
                let b_name = b.as_ref().unwrap().file_name();
                a_name
                    .to_ascii_lowercase()
                    .cmp(&b_name.to_ascii_lowercase())
            });
        }
        SortType::ByFileName => {
            entries.sort_unstable_by_key(|entry| entry.as_ref().unwrap().file_name());
        }
        SortType::NoSort => {}
    }
}
