use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/*
FIXME
Apply lazy evaluation where the default info we only need is the
- files name
- files size

Others is optional.
*/

pub struct FileInfo {
    pub name: String,
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

impl FileInfo {
    /// Collect information for each file/folder
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;
        let (is_symlink, symlink_target) = FileInfo::get_symlink_info(&full_path, &file_type);

        Ok(FileInfo {
            name: full_path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("Invalid full-path")
                .to_string(),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            mode: metadata.mode(),
            user_id: metadata.uid(),
            group_id: metadata.gid(),
            size: metadata.len(),
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

    fn get_symlink_info(path: &Path, file_type: &FileType) -> (bool, Option<String>) {
        if file_type.is_symlink() {
            if let Ok(target) = fs::read_link(path) {
                (true, Some(target.to_string_lossy().into_owned()))
            } else {
                (false, None)
            }
        } else {
            (false, None)
        }
    }
}
