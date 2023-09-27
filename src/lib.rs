pub mod engine;
pub mod format;
pub mod metada;
pub mod sort;
pub mod total;
pub mod flag;
use crate::{format::*, metada::*, total::*};
use colored::*;
use sort::sort::*;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {

    // let args: Vec<String> = env::args().collect();

    let directory_path = "/home/nemesis/Documents/Github/my_repo";
    let sort_type = SortType::ByLowerCaseFileName;

    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);

    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
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

    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();

    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {
        let info = FileInfo::new(&entry.as_ref().unwrap(), depth)?;

        if index < entries.len() - 1 {
            dynamic_places.push(1);
        } else {
            dynamic_places.push(2);
        };

        treestructureformatter.print_directory_structure(
            dynamic_places,
            dynamic_places.len() - 1,
            output,
        )?;

        if info.file_type.is_dir() {
            // FIXME: Create custom "printit"
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

        // Pop the last element to backtrack
        dynamic_places.pop();
    }

    Ok(())
}
