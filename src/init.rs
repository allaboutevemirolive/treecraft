use crate::config::Config;
use crate::config::ConfigInfo;
use crate::config::DisplayBrightGreen;
use crate::config::DisplayOsString;
use crate::flag::Flags;
use crate::fmt::TreeStructureFormatter;
use crate::handle::OutputHandler;
use crate::loc::PrintLocation;
use crate::sort::{sort_entries, Sort};
use crate::total::Totals;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

// TODO
// Create custom error message
#[allow(clippy::cognitive_complexity)]
#[cfg(any(unix, windows))]
pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let mut totals = Totals::new();
    let mut output_handler = output_writer(&flags.output)?;
    let start_time = Instant::now();

    walk_dirs(
        Path::new(&flags.dirname.to_string_lossy().into_owned()),
        // This is the primary location to determine the structure of the branch.
        &mut Vec::with_capacity(5_000),
        &1,
        &mut totals,
        &TreeStructureFormatter::new(),
        &mut output_handler,
        &flags.sorttype,
        &flags.output,
        &flags.config,
    )?;

    output_handler.flush()?;

    log_metrics(&mut output_handler, &totals, start_time)?;

    Ok(())
}

/// Traverse nested directories recursively
/// and gather information about each folder.
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
    set_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::{ConfigAll, ConfigDefault};

    let mut entries: Vec<_> = fs::read_dir(path)?.collect();

    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {
        // Modifying the current vector for generating tree branch
        if index < entries.len() - 1 {
            node_links.push(1);
        } else {
            node_links.push(2);
        };

        fmt.print_tree(node_links, node_links.len() - 1, output_handler)?;

        #[rustfmt::skip]
        let info = match set_config {
            Config::All => ConfigInfo::All(
                ConfigAll::new(&entry.as_ref().unwrap(), depth)?
            ),
            Config::Default => ConfigInfo::Default(
                ConfigDefault::new(&entry.as_ref().unwrap(), depth)?
            ),
        };

        handle_info(
            info,
            totals,
            fmt,
            output_handler,
            output_location,
            node_links,
            depth,
            sort_type,
            set_config,
        )?;

        node_links.pop();
    }

    Ok(())
}

#[allow(clippy::cognitive_complexity)]
#[inline(always)]
fn handle_info(
    info: ConfigInfo,
    totals: &mut Totals,
    fmt: &TreeStructureFormatter,
    output_handler: &mut OutputHandler,
    output_location: &PrintLocation,
    node_links: &mut Vec<i32>,
    depth: &i32,
    sort_type: &Sort,
    set_config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match info {
        ConfigInfo::All(info) => {
            if info.file_type.is_dir() {
                // Avoid ANSI color if printing in a file,
                // but include ANSI when printing to the terminal.
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
                    &set_config,
                )?;
            } else {
                writeln!(output_handler, "{}", DisplayOsString(info.name))?;
                totals.files += 1;
            }

            totals.size += info.size;
        }
        ConfigInfo::Default(info) => {
            if info.file_type.is_dir() {
                // Avoid ANSI color if printing in a file,
                // but include ANSI when printing to the terminal.
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
                    &set_config,
                )?;
            } else {
                writeln!(output_handler, "{}", DisplayOsString(info.name))?;
                totals.files += 1;
            }

            totals.size += info.size;
        }
    }

    Ok(())
}

/// Select the output type based on the provided flag.
///
/// Options include 'terminal' or 'textfile'.
fn output_writer(
    print_location: &PrintLocation,
) -> Result<OutputHandler, Box<dyn std::error::Error>> {
    match print_location {
        PrintLocation::File => {
            // TODO:
            // Allow the user to specify the output file.
            // If no output is defined, the default is 'Output.txt'.
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
