use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::os::unix::fs::MetadataExt; // For Unix-specific file metadata

#[derive(Debug)]
struct FileInfo {
    name: String,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u64,
    device_id: u64,
    inode: u64,
    is_directory: bool,
    is_symlink: bool,
    symlink_target: Option<String>,
    access_time: i64,
    change_time: i64,
    modification_time: i64,
}

fn read_dir(dir_path: &str) -> io::Result<Vec<FileInfo>> {
    let mut entries = Vec::new();

    for entry_result in fs::read_dir(dir_path)? {
        let entry = entry_result?;
        let path = entry.path();
        let file_name = entry.file_name();

        // Skip "." and ".." entries and hidden files
        if file_name == "." || file_name == ".." || file_name.to_string_lossy().into_owned().starts_with('.') {
            continue;
        }

        let metadata = fs::symlink_metadata(&path)?;
        let mut is_symlink = false;
        let mut symlink_target = None::<String>;

        // Check if the entry is a symbolic link
        if metadata.file_type().is_symlink() {
            is_symlink = true;
            if let Ok(target) = fs::read_link(&path) {
                symlink_target = Some(target.to_string_lossy().into_owned());
            }
        }

        let file_info = FileInfo {
            name: file_name.to_string_lossy().into_owned(),
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
        };

        entries.push(file_info);
    }

    Ok(entries)
}

fn main() -> io::Result<()> {
    let directory_path = "."; // Replace with the directory path you want to read
    let entries = read_dir(directory_path)?;

    for (index, entry) in entries.iter().enumerate() {
        println!("Entry {}:", index + 1);
        println!("  Name: {}", entry.name);
        println!("  Mode: {:o}", entry.mode);
        println!("  UID: {}", entry.uid);
        println!("  GID: {}", entry.gid);
        println!("  Size: {} bytes", entry.size);
        println!("  Device ID: {}", entry.device_id);
        println!("  Inode: {}", entry.inode);
        println!("  Is Directory: {}", entry.is_directory);
        println!("  Is Symbolic Link: {}", entry.is_symlink);
        if let Some(target) = &entry.symlink_target {
            println!("  Symlink Target: {}", target);
        }
        println!("  Access Time: {}", entry.access_time);
        println!("  Change Time: {}", entry.change_time);
        println!("  Modification Time: {}", entry.modification_time);
        println!();
    }

    Ok(())
}
