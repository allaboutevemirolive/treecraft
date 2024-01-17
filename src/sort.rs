use std::fs::DirEntry;
use std::io;

use crate::init::WalkDirs;

#[derive(Debug, Default)]
pub enum Sort {
    /// `-ci` Sort file's name with case-Insensitive
    CaseInsensitive,

    /// `-cs` Sort file's name with case-Sensitive
    #[default]
    CaseSensitive,

    /// `-n` No sort
    None,

    /// `-xt` sort based on file's extension
    Extension,
    //
    // TODO:
    // Size
    // Time
    // Version
}

/// If no, default sort, case-sensitive is used
#[inline(always)]
pub fn sort_ty(entries: &mut [Result<DirEntry, io::Error>], walker: &mut WalkDirs<'_>) {
    match walker.opts.sort_ty {
        Sort::CaseInsensitive => {
            entries.sort_unstable_by(|a, b| {
                let a_name = a.as_ref().unwrap().file_name();
                let b_name = b.as_ref().unwrap().file_name();
                a_name
                    .to_ascii_lowercase()
                    .cmp(&b_name.to_ascii_lowercase())
            });
        }
        // We follow the GNU ls's default sorting behaviour, case-sensitive
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
