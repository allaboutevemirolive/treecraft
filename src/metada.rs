use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

#[derive(Default)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub depth: i32,
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
    pub fn new(current_path: &str, depth: &i32) -> io::Result<Self> {
        let metadata = fs::symlink_metadata(current_path)?;

        // FIXME: We can initialize early.
        let mut file_info = FileInfo::default(); 

        file_info.name = Self::get_file_name(current_path);
        file_info.path = current_path.to_string();
        file_info.depth = *depth;
        file_info.mode = metadata.mode();
        file_info.user_id = metadata.uid();
        file_info.group_id = metadata.gid();
        file_info.size = metadata.len();
        file_info.device_id = metadata.dev();
        file_info.inode = metadata.ino();
        file_info.is_directory = metadata.is_dir();
        file_info.is_symlink = metadata.file_type().is_symlink();
        file_info.symlink_target = Self::get_symlink_target(current_path, &file_info.is_symlink);
        file_info.access_time = metadata.atime();
        file_info.change_time = metadata.ctime();
        file_info.modification_time = metadata.mtime();

        Ok(file_info)
    }

    fn get_file_name(path: &str) -> String {
        std::path::Path::new(path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    fn get_symlink_target(path: &str, is_symlink: &bool) -> Option<String> {
        if *is_symlink {
            fs::read_link(path)
                .ok()
                .and_then(|target| Some(target.to_string_lossy().into_owned()))
        } else {
            None
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
