pub mod file;
pub mod flag;
pub mod format;
pub mod meta;
pub mod output;
pub mod sort;
use crate::{flag::*, format::*};
use colored::*;
use file::file::OutputHandle;
use file::file::OutputType;
use meta::metada::FileInfo;
use meta::total::Totals;
use output::*;
use sort::sort::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const HELP_TEXT: [&str; 5] = [
    "-tf                         Print output in a text file",
    "-st-fn-lc                   Sort filename with case insensitive or lowercase",
    "-st-fn                      Sort filename",
    "-st-no                      No sort",
    "-help                       Print usage and exit",
];

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut flags = Flags::new();
    flags.processing_args(args);

    if flags.help {
        for line in &HELP_TEXT {
            println!("{}", line);
        }

        std::process::exit(0);
    }

    run_main(&flags)?;

    Ok(())
}

fn read_directory_recursive(
    path: &Path,
    dynamic_places: &mut Vec<i32>,
    depth: &i32,
    totals: &mut Totals,
    treestructureformatter: &TreeStructureFormatter,
    output: &mut OutputHandle,
    sort_type: &SortType,
    flags: &Flags,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();

    sort_entries(&mut entries, &sort_type);

    for (index, entry) in entries.iter().enumerate() {
        // Collect information for each file/folder
        let info = FileInfo::new(&entry.as_ref().unwrap(), depth)?;

        // Manipulate vector for branches creation
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

        // output.flush()?;

        if info.file_type.is_dir() {
            // FIXME: Create custom "printit" to handle unicode
            if flags.output == OutputType::File {
                writeln!(output, "{}", info.name)?;
            } else {
                writeln!(output, "{}", info.name.color(Color::BrightGreen))?;
            }

            totals.dirs += 1;

            read_directory_recursive(
                &info.path,
                dynamic_places,
                &(depth + 1),
                totals,
                treestructureformatter,
                output,
                &sort_type,
                &flags,
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
