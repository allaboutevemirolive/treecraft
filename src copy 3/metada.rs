use std::fs;
use std::fs::DirEntry;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::fs::FileType;

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
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;
        let (is_symlink, symlink_target) = FileInfo::get_symlink_info(&full_path, &file_type);

        Ok(FileInfo {
            name: full_path.file_name().and_then(|os_str| os_str.to_str()).unwrap_or("Unknown").to_string(),
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
            match fs::read_link(path) {
                Ok(target) => (true, Some(target.to_string_lossy().into_owned())),
                Err(_) => (false, None),
            }
        } else {
            (false, None)
        }
    }
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
