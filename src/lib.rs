pub mod engine;
pub mod format;
pub mod metada;
pub mod sort;
pub mod total;
use crate::{format::*, metada::*, total::*};
use colored::*;
use sort::sort::*;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo";

    /*
    ## Main place to determine the structure of branch

    FIXME: 
    Initialize fixed vector with 64_000 size and initialize to 0, 
    then replace push and pop with marker 1 and 2, 
    then use "depth" as the benchmark to modify marker to 0
    or update the marker
    
    This ways we can avoid amortized O(1) operation
    
    FIXME:
    Extract this logic into new file

     */
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);

    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let sort_type = SortType::ByLowerCaseFileName;

    // Measure execution time
    let start_time = Instant::now();

    read_directory_recursive(
        Path::new(directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut handle,
        &sort_type,
    )
    .unwrap();

    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    println!();
    println!("Times Processing  : {:?}s", seconds);
    println!("Total Folders     : {}", totals.dirs);
    println!("Total Files       : {}", totals.files);
    println!("Total Size        : {} bytes", totals.size);
    println!();

    Ok(())
}

fn read_directory_recursive(
    path: &Path,
    dynamic_places: &mut Vec<i32>,
    depth: &i32,
    totals: &mut Totals,
    treestructureformatter: &TreeStructureFormatter,
    output: &mut dyn Write,
    sort_type: &SortType,
) -> Result<(), Box<dyn std::error::Error>> {
    // FIXME:
    // We can split our findings into several part
    // and use rayon for parallel process

    // FIXME:
    // Need to custom sort
    // We need to define sort where
    // - Files is first
    // - Folder is first
    //
    // FIXME:
    // Extract this logic into new file
    //
    // FIXME:
    // When sorting, sort it with case insensitive or make it as option

    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();

    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {
        let info = FileInfo::new(&entry.as_ref().unwrap(), depth)?;

        // FIXME:
        // We need to work around for push and pop
        // Like, use fix arrays initialized with 0,
        // then use 1 and 2 as the marker, 0 as pop,
        // Use depth/index as the index we need to modify
        // the vector
        //
        // FIXME:
        // Extract this logic into new file
        if index < entries.len() - 1 {
            dynamic_places.push(1);
        } else {
            dynamic_places.push(2);
        };

        // FIXME:
        //
        // Use std::io ?
        //
        // For better memory-efficient, we may use vector,
        // but it also mean we need to handle character encoding
        // and decoding manually. Thus, we need to use "custom printit".
        // Another resonale for "custom printit" is that we need
        // to handle unicode char in folder list that we find
        //
        // let mut outfile = String::from("");

        treestructureformatter.print_directory_structure(
            dynamic_places,
            dynamic_places.len() - 1,
            output,
        )?;

        // FIXME:
        // Maybe put this call inside "print_directory_structure"?
        // Will replace it with custom print
        // println!("{}{}", outfile, info.name);
        // writeln!(output, "{}", info.name,)?;

        if info.file_type.is_dir() {
            writeln!(output, "{}", info.name.color(Color::BrightGreen))?;
            totals.dirs += 1;
            read_directory_recursive(
                &info.path,
                dynamic_places,
                &(depth + 1),
                totals,
                treestructureformatter,
                output,
                &sort_type,
            )?;
        } else {
            writeln!(output, "{}", info.name,)?;
            totals.files += 1;
        }

        totals.size += info.size;

        // FIXME:
        // Extract this logic into new file
        //
        // Pop the last element to backtrack
        dynamic_places.pop();
    }

    Ok(())
}
