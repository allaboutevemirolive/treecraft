use std::fs::DirEntry;
use std::io;

pub enum SortType {
    ByLowerCaseFileName,
    ByFileName,
}

pub fn sort_entries(entries: &mut Vec<Result<DirEntry, io::Error>>, sort_type: &SortType) {
    match sort_type {
        SortType::ByLowerCaseFileName => {
            entries.sort_unstable_by_key(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .to_lowercase()
            });
        }
        SortType::ByFileName => {
            entries.sort_unstable_by_key(|entry| entry.as_ref().unwrap().file_name());
        }
    }
}
