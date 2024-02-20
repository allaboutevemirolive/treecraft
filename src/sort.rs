use std::fs::DirEntry;
use std::io;

use crate::walker::WalkDir;

#[derive(Default)]
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

// TODO: Add sort number sensitive & etc. ...
/// If no sort specified, sort case-sensitive is used
#[inline(always)]
pub fn ty_sort(entries: &mut [Result<DirEntry, io::Error>], walk: &mut WalkDir<'_>) {
    match walk.flag.sort_ty {
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
