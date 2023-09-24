use std::{fs, io};
use std::os::unix::fs::MetadataExt;
use std::path::Path; // For Unix-specific file metadata

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub level: i32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
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
    pub fn new(target_path: &str, level: i32) -> Self {

        let metadata = fs::symlink_metadata(target_path).unwrap();

        let mut is_symlink = false;
        let mut symlink_target = None::<String>;

        // Get the filename from the target_path
        let filename = Path::new(target_path)
            .file_name()
            .map(|os_str| os_str.to_string_lossy().into_owned())
            .unwrap_or_else(|| "".to_string());

        // Check if the entry is a symbolic link
        if metadata.file_type().is_symlink() {
            is_symlink = true;
            if let Ok(target) = fs::read_link(target_path) {
                symlink_target = Some(target.to_string_lossy().into_owned());
            }
        }

        Self {
            name: filename,
            path: target_path.to_string(),
            level,
            mode: metadata.mode(),
            uid: metadata.uid(),
            gid: metadata.gid(),
            size: metadata.len(),
            device_id: metadata.dev(),
            inode: metadata.ino(),
            is_directory: metadata.is_dir(),
            is_symlink,
            symlink_target,
            access_time: metadata.atime(),
            change_time: metadata.ctime(),
            modification_time: metadata.mtime(),
        }
    }
}

// pub fn retrieve_metadata(file_or_dir_path: &str, level: i32) -> io::Result<FileInfo> {
//     // let metadata = fs::symlink_metadata(file_or_dir_path)?;
//     Ok(FileInfo::new(file_or_dir_path, &metadata, level))
// }