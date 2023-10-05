use crate::file::file::{OutputHandler, PrintLocation};
use crate::flag::Flags;
use crate::format::TreeStructureFormatter;
use crate::meta::total::Totals;
use crate::walk_directories;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;

    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
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

/// Choose the output type based on the provided flag.
///
/// Etc. `terminal` or `textfile`
fn output_writer(print_location: &PrintLocation) -> Result<OutputHandler, Box<dyn std::error::Error>> {
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
    writeln!(output_handler, "Total Items       : {}", totals.files + totals.directories)?;
    writeln!(
        output_handler,
        "Total Size        : {:.2} GB or {} bytes",
        gigabytes, totals.size
    )?;
    writeln!(output_handler)?;

    Ok(())
}
