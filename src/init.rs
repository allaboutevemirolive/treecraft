use crate::file::file::{OutputHandle, OutputType};
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

    let mut handle = output_writer(&flags.output)?;

    let start_time = Instant::now();

    walk_directories(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut handle,
        &sort_type,
        &flags,
    )
    .unwrap_or_default();

    handle.flush()?;

    // Since `Instant` implements the `Copy` trait, we
    // can pass it by value without cloning or using reference '&'
    log_metrics(&mut handle, &totals, start_time)?;

    Ok(())
}

/// Choose the output type based on the provided flag.
///
/// Etc. `terminal` or `textfile`
fn output_writer(output_type: &OutputType) -> Result<OutputHandle, Box<dyn std::error::Error>> {
    match output_type {
        OutputType::File => {
            let output_file = File::create("Output.txt")?;
            let file_writer = BufWriter::new(output_file);
            let file_writer_refcell = Rc::new(RefCell::new(file_writer));
            Ok(OutputHandle::new(file_writer_refcell))
        }
        OutputType::Stdout => {
            let stdout = io::stdout();
            let stdout_writer = BufWriter::new(stdout.lock());
            let stdout_writer_refcell = Rc::new(RefCell::new(stdout_writer));
            Ok(OutputHandle::new(stdout_writer_refcell))
        }
    }
}

fn log_metrics(
    handle: &mut OutputHandle,
    totals: &Totals,
    start_time: Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    let gigabytes = totals.size as f64 / 1_073_741_824.0;

    writeln!(handle)?;
    writeln!(handle, "Times Processing  : {:?}s", seconds)?;
    writeln!(handle, "Total Folders     : {}", totals.dirs)?;
    writeln!(handle, "Total Files       : {}", totals.files)?;
    writeln!(handle, "Total Items       : {}", totals.files + totals.dirs)?;
    writeln!(
        handle,
        "Total Size        : {:.2} GB or {} bytes",
        gigabytes, totals.size
    )?;
    writeln!(handle)?;

    Ok(())
}
