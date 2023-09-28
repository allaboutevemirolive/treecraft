use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    time::Instant,
};

use crate::{
    flag::Flags, format::TreeStructureFormatter, read_directory_recursive, sort::sort::SortType,
    total::Totals,
};

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Terminal,
    TextFile,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::Terminal
    }
}

pub fn run_terminal(flags: Flags) -> Result<(), Box<dyn std::error::Error>> {
    // HardCode
    // let directory_path = "/home/nemesis/Documents/Github/Focus/lang";
    // let sort_type = SortType::ByLowerCaseFileName;

    let directory_path = flags.dirname.to_string();
    let sort_type = &flags.sorttype;

    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);

    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let start_time = Instant::now();

    read_directory_recursive(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut handle,
        &sort_type,
        &flags,
    )
    .unwrap();

    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    let gigabytes = totals.size as f64 / 1_073_741_824.0;

    println!();
    println!("Times Processing  : {:?}s", seconds);
    println!("Total Folders     : {}", totals.dirs);
    println!("Total Files       : {}", totals.files);
    println!("Total Items       : {}", totals.files + totals.dirs);
    println!("Total Size        : {:.2} GB or {} bytes",gigabytes, totals.size);
    println!();

    Ok(())
}

pub fn run_text_file(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;

    // Create a text file to write the output
    let output_file_path = "output.txt";
    let output_file = File::create(&output_file_path)?;

    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();

    // Redirect stdout to the output file
    let stdout = io::stdout();
    let _stdout_guard = stdout.lock(); // Lock stdout and keep it in a variable to limit its scope
    io::stdout().flush()?;
    io::stdout().lock().flush()?;

    let start_time = Instant::now();

    // Redirect stdout to the output file
    let mut handle = io::BufWriter::new(&output_file);

    read_directory_recursive(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut handle,
        &sort_type,
        &flags,
    )?;

    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    let gigabytes = totals.size as f64 / 1_073_741_824.0;

    writeln!(&mut handle)?;
    writeln!(&mut handle, "Times Processing  : {:?}s", seconds)?;
    writeln!(&mut handle, "Total Folders     : {}", totals.dirs)?;
    writeln!(&mut handle, "Total Files       : {}", totals.files)?;
    writeln!(
        &mut handle,
        "Total Items       : {}",
        totals.files + totals.dirs
    )?;
    writeln!(
        &mut handle,
        "Total Size        : {:.2} GB or {} bytes",
        gigabytes, totals.size
    )?;

    drop(handle);

    // Close the output file
    drop(output_file);

    Ok(())
}
