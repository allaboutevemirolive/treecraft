pub mod file_info;
pub mod metada;
pub mod print_branch;
use std::fs;
use std::io;
use std::path::Path;

use crate::{metada::*, print_branch::*};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    
    let directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft";
    let mut dynamic_places: Vec<i32> = Vec::new();
    read_directory_recursive(Path::new(directory_path), &mut dynamic_places)?;

    Ok(())
}

fn read_directory_recursive(path: &Path, dynamic_places: &mut Vec<i32>) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?.collect();

    entries.sort_by_key(|entry| entry.as_ref().unwrap().file_name());

    for (index, entry) in entries.iter().enumerate() {
        let entry = entry.as_ref().unwrap();
        let file_type = entry.file_type()?;
        let full_path = entry.path();

        let info = retrieve_metadata(&full_path.to_string_lossy())?;

        if index < entries.len() - 1 {
            dynamic_places.push(1);
        } else {
            dynamic_places.push(2);
        };

        let treestructureformatter = TreeStructureFormatter::new();
        let mut outfile = String::from("");

        let maxlevel = dynamic_places.len() - 1;
        treestructureformatter.print_directory_structure(
            dynamic_places,
            maxlevel,
            &mut outfile,
        );

        // Will replace it with custom print
        println!("{}{}", outfile, info.name);

        if file_type.is_dir() {
            read_directory_recursive(&full_path, dynamic_places)?;
        }

        // Pop the last element to backtrack
        dynamic_places.pop();
    }

    Ok(())
}
