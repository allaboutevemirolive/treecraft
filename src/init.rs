use crate::flag::Flags;
use crate::fmt::TreeStructureFormatter;
use crate::handler::{OutputHandler, PrintLocation};
use crate::meta::total::Totals;
use crate::meta::FileInfo;
use crate::sort::{sort_entries, SortType};
use colored::*;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

// Fixme
// Create custom error message
pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;

    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(5000);

    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();

    let mut output_handler = output_writer(&flags.output)?;

    let start_time = Instant::now();

    walk_directories(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut output_handler,
        &sort_type,
        &flags,
    )
    .unwrap_or_default();

    output_handler.flush()?;

    // `Instant` implements the `Copy` trait, so we
    // can pass it by value without cloning or using reference '&'
    log_metrics(&mut output_handler, &totals, start_time)?;

    Ok(())
}

/// Recursively traverse nested directories while
/// gathering information about each folder.
fn walk_directories(
    path: &Path,
    dynamic_places: &mut Vec<i32>,
    depth: &i32,
    totals: &mut Totals,
    formatter: &TreeStructureFormatter,
    output_handler: &mut OutputHandler,
    sort_type: &SortType,
    flags: &Flags,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();
    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {
        let info = FileInfo::new(&entry.as_ref().unwrap(), depth)?;

        // Marking current vector to generate branch
        if index < entries.len() - 1 {
            dynamic_places.push(1);
        } else {
            dynamic_places.push(2);
        };

        formatter.print_tree(dynamic_places, dynamic_places.len() - 1, output_handler)?;
        // output_handler.flush()?;
        if info.file_type.is_dir() {
            // FIXME:
            // Check if Rust can handle different unicode
            // in different OS. If cannot,
            // create custom "printit" to handle unicode
            if flags.output == PrintLocation::File {
                writeln!(output_handler, "{}", info.name)?;
            } else {
                writeln!(output_handler, "{}", info.name.color(Color::BrightGreen))?;
            }

            totals.directories += 1;

            walk_directories(
                &info.path,
                dynamic_places,
                &(depth + 1),
                totals,
                formatter,
                output_handler,
                &sort_type,
                &flags,
            )?;
        } else {
            writeln!(output_handler, "{}", info.name,)?;
            totals.files += 1;
        }

        totals.size += info.size;
        dynamic_places.pop();
    }

    Ok(())
}

/// Choose the output type based on the provided flag.
///
/// Etc. `terminal` or `textfile`
fn output_writer(
    print_location: &PrintLocation,
) -> Result<OutputHandler, Box<dyn std::error::Error>> {
    match print_location {
        PrintLocation::File => {
            let output_file = File::create("Output.txt")?;
            let file_writer = BufWriter::new(output_file);
            let file_writer_refcell = Rc::new(RefCell::new(file_writer));
            Ok(OutputHandler::new(file_writer_refcell))
        }
        PrintLocation::Stdout => {
            let stdout = io::stdout();
            let stdout_writer = BufWriter::new(stdout.lock());
            let stdout_writer_refcell = Rc::new(RefCell::new(stdout_writer));
            Ok(OutputHandler::new(stdout_writer_refcell))
        }
    }
}

fn log_metrics(
    output_handler: &mut OutputHandler,
    totals: &Totals,
    start_time: Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    let gigabytes = totals.size as f64 / 1_073_741_824.0;

    writeln!(output_handler)?;
    writeln!(output_handler, "Times Processing  : {:?}s", seconds)?;
    writeln!(output_handler, "Total Directories : {}", totals.directories)?;
    writeln!(output_handler, "Total Files       : {}", totals.files)?;
    writeln!(
        output_handler,
        "Total Items       : {}",
        totals.files + totals.directories
    )?;
    writeln!(
        output_handler,
        "Total Size        : {:.2} GB or {} bytes",
        gigabytes, totals.size
    )?;
    writeln!(output_handler)?;

    Ok(())
}
