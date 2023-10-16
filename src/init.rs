use crate::flag::Flags;
use crate::fmt::TreeStructureFormatter;
use crate::handler::{OutputHandler, PrintLocation};
use crate::config::total::Totals;
use crate::config::Config;
use crate::config::DisplayOsString;
use crate::config::DisplayBrightGreen;
use crate::sort::{sort_entries, Sort};
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

// Fixme
// Create custom error message
#[allow(clippy::cognitive_complexity)]
#[cfg(any(unix, windows))]
pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let mut totals = Totals::new();
    let mut output_handler = output_writer(&flags.output)?;
    let start_time = Instant::now();

    walk_dirs(
        Path::new(&flags.dirname.to_string_lossy().into_owned()),
        // TODO: Need more explanation how this works
        // Main place to determine the structure of branch
        &mut Vec::with_capacity(5_000),
        &1,
        &mut totals,
        &TreeStructureFormatter::new(),
        &mut output_handler,
        &flags.sorttype,
        &flags.output,
    )?;

    output_handler.flush()?;

    // `Instant` implements the `Copy` trait, so we
    // can pass it by value without cloning or using reference '&'
    log_metrics(&mut output_handler, &totals, start_time)?;

    Ok(())
}

/// Recursively traverse nested directories while
/// gathering information about each folder.
#[cfg(any(unix, windows))]
#[allow(clippy::cognitive_complexity)]
#[inline(always)]
fn walk_dirs(
    path: &Path,
    node_links: &mut Vec<i32>,
    depth: &i32,
    totals: &mut Totals,
    fmt: &TreeStructureFormatter,
    output_handler: &mut OutputHandler,
    sort_type: &Sort,
    output_location: &PrintLocation,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir(path)?.collect();

    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {

        // DEBUG:
        // println!("entry: {:?}", entry.as_ref());

        let info = Config::new(&entry.as_ref().unwrap(), depth)?;

        // TODO: More explanation how this works
        // Marking current vector to generate branch
        if index < entries.len() - 1 {
            node_links.push(1);
        } else {
            node_links.push(2);
        };

        // FIXME
        fmt.print_tree(node_links, node_links.len() - 1, output_handler)?;

        // DEBUG
        // output_handler.flush()?;

        if info.file_type.is_dir() {

            // If print in file, avoid ansi color
            // if print on terminal, include ansi
            if output_location == &PrintLocation::File {
                writeln!(output_handler, "{}", DisplayOsString(info.name))?;
            } else {
                writeln!(output_handler, "{}", DisplayBrightGreen(info.name))?;
            }

            totals.directories += 1;
            walk_dirs(
                &info.path,
                node_links,
                &(depth + 1),
                totals,
                fmt,
                output_handler,
                &sort_type,
                &output_location,
            )?;
        } else {
            writeln!(output_handler, "{}", DisplayOsString(info.name))?;
            totals.files += 1;
        }

        totals.size += info.size;
        node_links.pop();
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
            // FIXME
            // Let user define the output file
            // If no output where defined, by default
            // use 'Output.txt'
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
