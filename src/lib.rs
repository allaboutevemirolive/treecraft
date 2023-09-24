pub mod format;
pub mod metada;
pub mod total;
use std::fs;
use std::io;
use std::path::Path;

use crate::{format::*, metada::*, total::*};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft";

    // FIXME: Initialize to fix vector, flexible vector is expensive
    let mut dynamic_places: Vec<i32> = Vec::new();

    let depth = 1;

    let mut totals = Totals::new();

    let treestructureformatter = TreeStructureFormatter::new();

    read_directory_recursive(
        Path::new(directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
    )?;


    println!("Total Directories : {}", totals.dirs);
    println!("Total Files       : {}", totals.files);
    println!("Total Size        : {} bytes", totals.size);

    Ok(())
}

fn read_directory_recursive(
    path: &Path,
    dynamic_places: &mut Vec<i32>,
    depth: &i32,
    totals: &mut Totals,
    treestructureformatter: &TreeStructureFormatter,
) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?.collect();

    // FIXME: Custom sort
    entries.sort_unstable_by_key(|entry| entry.as_ref().unwrap().file_name());

    for (index, entry) in entries.iter().enumerate() {
        let entry = entry.as_ref().unwrap();
        let file_type = entry.file_type()?;
        let full_path = entry.path();

        let info = FileInfo::new(&full_path.to_string_lossy(), &depth)?;

        if index < entries.len() - 1 {
            dynamic_places.push(1);
        } else {
            dynamic_places.push(2);
        };

        // FIXME: Use std::io
        let mut outfile = String::from("");
        let maxlevel = dynamic_places.len() - 1;

        treestructureformatter.print_directory_structure(dynamic_places, maxlevel, &mut outfile);

        // Will replace it with custom print
        println!("{}{}", outfile, info.name);

        if file_type.is_dir() {
            totals.dirs += 1;
            read_directory_recursive(
                &full_path,
                dynamic_places,
                &(depth + 1),
                totals,
                treestructureformatter,
            )?;
        } else {
            totals.files += 1;
        }

        totals.size += info.size;

        // Pop the last element to backtrack
        dynamic_places.pop();
    }

    Ok(())
}
