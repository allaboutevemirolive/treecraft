use std::error::Error;
use std::fs;

#[derive(Debug, Default, Clone)]
struct FileInfo {
    name: String,
    level: i32,
    is_dir: bool,
    // If it is folder and has file in it, put 1. Same as "descend"
    // We need this to make newline for the next branch
    has_subdirectory: i32,
}

impl FileInfo {
    fn new() -> Self {
        Self::default()
    }
}

fn listdir(path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut entries: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path().to_string_lossy().to_string();
        entries.push((file_name, path));
    }
    Ok(entries)
}

fn read_directory_recursive(dir_path: &str, level: i32) -> Result<Vec<FileInfo>, Box<dyn Error>> {
    let mut info_vec: Vec<FileInfo> = Vec::new();
    let entries = listdir(dir_path)?;

    for (_, path) in entries {
        let metadata = fs::metadata(&path)?;

        let mut file_info = FileInfo::new();
        file_info.name = path.clone(); // Use the path as the name for now
        file_info.is_dir = metadata.is_dir();
        file_info.level = level;

        if file_info.is_dir {
            // let nested_info_vec = read_directory_recursive(&path, level + 1)?;
            // if nested_info_vec.iter().any(|info| !info.is_dir || info.is_dir) {
            //     file_info.has_subdirectory = 1;
            // }
            info_vec.push(file_info.clone());
            // info_vec.extend(nested_info_vec);
        } else {
            info_vec.push(file_info.clone());
        }
    }

    Ok(info_vec)
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory_path = "/home/nemesis/Documents/Github/test/trie/read_dir";

    let level = 1;

    match read_directory_recursive(directory_path, level) {
        Ok(info_vec) => {
            if info_vec.is_empty() {
                println!("No files found in the directory.");
            } else {
                for info in &info_vec {
                    println!("Name: {}", info.name);
                    println!("Level: {}", info.level);
                    println!("has_subdirectory: {}", info.has_subdirectory);
                    println!("Is Directory: {}", info.is_dir);
                    println!("----------------------------------");
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
