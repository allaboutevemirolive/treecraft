use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo/DynamicTreeView/test_folder";
    let mut dynamic_places: Vec<i32> = Vec::new();

    read_directory_recursive(Path::new(directory_path), &mut dynamic_places)?;

    Ok(())
}

fn read_directory_recursive(
    path: &Path,
    dynamic_places: &mut Vec<i32>,
) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?.collect(); // Collect entries into a Vec

    // Sort the entries by file name
    entries.sort_by_key(|entry| entry.as_ref().unwrap().file_name());

    for (index, entry) in entries.iter().enumerate() {
        let entry = entry.as_ref().unwrap(); // Unwrap the Result
        let file_type = entry.file_type()?;
        let full_path = entry.path();

        // Set the current level in dynamic_places
        dynamic_places.push(if index < entries.len() - 1 { 1 } else { 2 });

        // Print the dynamic_places vector
        println!("{:?}", dynamic_places);

        if file_type.is_dir() {
            read_directory_recursive(&full_path, dynamic_places)?;
        }

        // Pop the last element to backtrack
        dynamic_places.pop();
    }

    Ok(())
}
