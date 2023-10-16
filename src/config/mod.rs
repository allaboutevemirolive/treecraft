pub mod total;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

// FIXME
// Apply lazy evaluation where the default info we only need is the:
// - files name
// - files size
//
// Others is optional.
#[derive()]
pub struct Config {
    pub name: OsString,
    pub path: PathBuf,
    pub depth: i32,
    pub file_type: FileType,
    pub mode: u32,
    pub user_id: u32,
    pub group_id: u32,
    pub size: u64,
    pub device_id: u64,
    pub inode: u64,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub symlink_target: Option<String>,
    pub access_time: i64,
    pub change_time: i64,
    pub modification_time: i64,
}

pub struct DisplayOsString(pub OsString);

impl fmt::Display for DisplayOsString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

/// Intended for terminal output with `ANSI`
pub struct DisplayBrightGreen(pub OsString);

impl fmt::Display for DisplayBrightGreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the string with the bright green color,
        // insert OsString, then reset color to none
        write!(
            f,
            "{}{}{}",
            "\x1b[32m",
            self.0.to_string_lossy(),
            "\x1b[0m"
        )
    }
}

impl Config {
    #[inline(always)]
    /// Collect information for each file/folder
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        let (is_symlink, symlink_target) = Config::get_symlink_info(&full_path, &file_type);

        Ok(Config {
            name: full_path
                // .file_name()
                // .and_then(|os_str| os_str.to_str())
                // .unwrap_or("Invalid full-path")
                // .to_string().into(),
                .file_name()
                .map(|os_str| os_str.to_os_string())
                .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            size: metadata.len(),

            mode: metadata.mode(),
            user_id: metadata.uid(),
            group_id: metadata.gid(),

            device_id: metadata.dev(),
            inode: metadata.ino(),
            is_directory: metadata.is_dir(),
            is_symlink,
            symlink_target,
            access_time: metadata.atime(),
            change_time: metadata.ctime(),
            modification_time: metadata.mtime(),
        })
    }

    #[inline(always)]
    // FIXME: Check if we need to convert this into OsString
    fn get_symlink_info(path: &Path, file_type: &FileType) -> (bool, Option<String>) {
        if file_type.is_symlink() {
            match fs::read_link(path) {
                Ok(target) => (true, Some(target.to_string_lossy().into_owned())),
                Err(_) => (false, None),
            }
        } else {
            (false, None)
        }
    }
}
