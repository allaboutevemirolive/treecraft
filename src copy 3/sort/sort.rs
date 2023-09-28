use std::fs::DirEntry;
use std::io;

#[derive(Debug)]
pub enum SortType {
    
    // -st-lc
    ByLowerCaseFileName,

    /*
    -st-fn

    This is much faster
    FIXME
    Make this as default
     */
    ByFileName,

    /*
    -st-no
     */
    NoSort,
}

impl Default for SortType {
    fn default() -> Self {
        SortType::ByLowerCaseFileName // Default value
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
