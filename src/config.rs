use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Config {
    All,
    Default,
}

impl Default for Config {
    fn default() -> Self {
        Config::Default
    }
}

/// This wrapper enables the return of different types.
/// 
/// It's used to provide default fields without recalculating everything.
#[derive(Debug)]
pub enum ConfigInfo {
    All(ConfigAll),
    /// Gather basic information
    Default(ConfigDefault),
}

#[derive(Debug)]
pub struct ConfigAll {
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

/// Meant for output in a terminal with ANSI support
pub struct DisplayBrightGreen(pub OsString);

impl fmt::Display for DisplayBrightGreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Apply bright green color formatting to the string, 
        // insert the OsString, and reset the color to default
        write!(
            f,
            "{}{}{}",
            "\x1b[1;32m",
            self.0.to_string_lossy(),
            "\x1b[0m"
        )
    }
}

impl ConfigAll {
    #[inline(always)]
    /// Gather data for each file or folder
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        let (is_symlink, symlink_target) = ConfigAll::get_symlink_info(&full_path, &file_type);

        Ok(ConfigAll {
            name: full_path
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
    // FIXME: Verify if converting this to an OsString is necessary.
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

#[derive(Debug)]
pub struct ConfigDefault {
    pub name: OsString,
    pub path: PathBuf,
    pub depth: i32,
    pub file_type: FileType,
    pub size: u64,
}

impl ConfigDefault {
    #[inline(always)]
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        Ok(ConfigDefault {
            name: full_path
                .file_name()
                .map(|os_str| os_str.to_os_string())
                .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            size: metadata.len(),
        })
    }
}
