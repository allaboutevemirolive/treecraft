use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
struct FileInfo {
    name: String,
    path: PathBuf, // Add a path field
    level: i32,
    is_dir: bool,
    has_subdirectory: bool,
}

impl FileInfo {
    fn new(name: String, path: PathBuf, level: i32, is_dir: bool, has_subdirectory: bool) -> Self {
        Self {
            name,
            path, // Set the path field
            level,
            is_dir,
            has_subdirectory,
        }
    }
}

fn get_list_dir(dir_path: &str, level: i32) -> Result<Vec<FileInfo>, Box<dyn Error>> {
    let mut info_vec: Vec<FileInfo> = Vec::new();
    let mut has_subdirectory = false;

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

        let mut file_info = FileInfo::new(
            file_name.to_string_lossy().to_string(),
            path.clone(), // Set the path
            level,
            is_dir,
            false, // Initially set has_subdirectory to false
        );

        info_vec.push(file_info.clone());

        if is_dir {
            // Don't recursively process subdirectories
            file_info.has_subdirectory = true;
        }
    }

    Ok(info_vec)
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft/backup_1";
    let level = 1;

    let sub_directory = get_list_dir(directory_path, level)?;

    for info in &sub_directory {
        println!("Name: {}", info.name);
        println!("Path: {:?}", info.path); // Print the path
        println!("Level: {}", info.level);
        println!("has_subdirectory: {}", info.has_subdirectory);
        println!("Is Directory: {}", info.is_dir);
        println!("----------------------------------");
    }

    Ok(())
}
