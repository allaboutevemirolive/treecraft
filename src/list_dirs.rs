use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct ListDirs {
    pub name: String,
    pub path: PathBuf,
}

impl ListDirs {
    fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

pub fn read_dir(dir_path: &str) -> Result<Vec<ListDirs>, Box<dyn Error>> {
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

        let file_info = ListDirs::new(file_name.to_string_lossy().to_string(), path.clone());

        info_vec.push(file_info.clone());
    }

    Ok(info_vec)
}
