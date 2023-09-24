use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt; // For Unix-specific file metadata

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
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
    pub fn new(current_path: &str, metadata: &fs::Metadata) -> Self {
        let mut is_symlink = false;
        let mut symlink_target = None::<String>;

        // Check if the entry is a symbolic link
        if metadata.file_type().is_symlink() {
            is_symlink = true;
            if let Ok(target) = fs::read_link(current_path) {
                symlink_target = Some(target.to_string_lossy().into_owned());
            }
        }

        let file_name = std::path::Path::new(&current_path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("Unknown");

        Self {
            name: file_name.to_string(),
            path: current_path.to_string(),
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

pub fn retrieve_metadata(file_or_dir_path: &str) -> io::Result<FileInfo> {
    let metadata = fs::symlink_metadata(file_or_dir_path)?;
    Ok(FileInfo::new(file_or_dir_path, &metadata))
}

// fn main() -> io::Result<()> {
//     let file_or_directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft/read_dir"; // Replace with the file or directory path you want to inspect
//     let info = retrieve_metadata(file_or_directory_path)?;

//     println!("File/Directory Info:");
//     println!("  Name: {}", info.name);
//     println!("  Mode: {:o}", info.mode);
//     println!("  UID: {}", info.uid);
//     println!("  GID: {}", info.gid);
//     println!("  Size: {} bytes", info.size);
//     println!("  Device ID: {}", info.device_id);
//     println!("  Inode: {}", info.inode);
//     println!("  Is Directory: {}", info.is_directory);
//     println!("  Is Symbolic Link: {}", info.is_symlink);
//     if let Some(target) = &info.symlink_target {
//         println!("  Symlink Target: {}", target);
//     }
//     println!("  Access Time: {}", info.access_time);
//     println!("  Change Time: {}", info.change_time);
//     println!("  Modification Time: {}", info.modification_time);

//     Ok(())
// }
