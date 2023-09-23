use std::error::Error;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf}; // For Unix-specific file metadata

#[derive(Debug, Clone, Default)]
struct ListDirs {
    name: String,
    path: PathBuf,
}

impl ListDirs {
    fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

fn read_dir(dir_path: &str) -> Result<Vec<ListDirs>, Box<dyn Error>> {
    let mut info_vec: Vec<ListDirs> = Vec::new();

    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return Ok(info_vec);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                continue;
            }
        };

        let file_name = entry.file_name();
        let path = entry.path();
        let is_dir = path.is_dir();

        let mut file_info = ListDirs::new(file_name.to_string_lossy().to_string(), path.clone());

        info_vec.push(file_info.clone());
    }

    Ok(info_vec)
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    path: String,
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

impl FileInfo {
    fn new(target_path: &str, metadata: &fs::Metadata) -> Self {
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

fn retrieve_metadata(file_or_dir_path: &str) -> io::Result<FileInfo> {
    let metadata = fs::symlink_metadata(file_or_dir_path)?;
    Ok(FileInfo::new(file_or_dir_path, &metadata))
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft/backup_1";

    let sub_directory = read_dir(directory_path)?;

    for info in &sub_directory {
        let info = retrieve_metadata((info.path).to_str().unwrap())?;

        println!("File/Directory Info:");
        println!("Name: {}", info.name);
        println!("Path: {}", info.path);
        println!("Mode: {:o}", info.mode);
        println!("UID: {}", info.uid);
        println!("GID: {}", info.gid);
        println!("Size: {} bytes", info.size);
        println!("Device ID: {}", info.device_id);
        println!("Inode: {}", info.inode);
        println!("Is Directory: {}", info.is_directory);
        println!("Is Symbolic Link: {}", info.is_symlink);
        if let Some(target) = &info.symlink_target {
            println!("Symlink Target: {}", target);
        }
        println!("Access Time: {}", info.access_time);
        println!("Change Time: {}", info.change_time);
        println!("Modification Time: {}", info.modification_time);

        println!("======================================");
    }

    Ok(())
}
