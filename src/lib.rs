pub mod file;
pub mod flag;
pub mod format;
pub mod init;
pub mod meta;
pub mod sort;
use crate::{flag::*, format::*};
use colored::*;
use file::file::OutputHandle;
use file::file::OutputType;
use meta::metada::FileInfo;
use meta::total::Totals;
use sort::sort::*;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Recursively traverse nested directories while
/// gathering information about each folder.
fn walk_directories(
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
        let info = FileInfo::new(&entry.as_ref().unwrap(), depth)?;

        // Marking current vector to generate branch
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
            // FIXME:
            // Check if Rust can handle different unicode
            // in different OS. If cannot,
            // create custom "printit" to handle unicode
            if flags.output == OutputType::File {
                writeln!(output, "{}", info.name)?;
            } else {
                writeln!(output, "{}", info.name.color(Color::BrightGreen))?;
            }

            totals.dirs += 1;

            walk_directories(
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
        dynamic_places.pop();
    }

    Ok(())
}
