pub mod engine;
pub mod flag;
pub mod format;
pub mod metada;
pub mod output;
pub mod sort;
pub mod total;
use crate::{flag::*, format::*, metada::*, total::*};
use colored::*;
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

    match flags.output {
        OutputType::Terminal => {
            run_terminal(&flags)?;
        }
        OutputType::TextFile => {
            run_text_file(&flags)?;
        }
    }

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

        if info.file_type.is_dir() {
            // FIXME: Create custom "printit" to handle unicode
            if flags.output == OutputType::TextFile {
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

// pub fn run() -> Result<(), Box<dyn std::error::Error>>{
//     // let args: Vec<String> = env::args().collect();
//     let mut flags = Flags::new();
//     // flags.processing_args(args);

//     // HardCode
//     let directory_path = "/home/nemesis/Documents/Github/Focus/lang";
//     let sort_type = SortType::ByLowerCaseFileName;

//     // let directory_path = flags.dirname.to_string();
//     // let sort_type = flags.sorttype;

//     // Main place to determine the structure of branch
//     let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);

//     let depth = 1;
//     let mut totals = Totals::new();
//     let treestructureformatter = TreeStructureFormatter::new();

//     // ---------------
//     let stdout = io::stdout();
//     let mut handle = stdout.lock();

//     let start_time = Instant::now();

//     read_directory_recursive(
//         Path::new(&directory_path),
//         &mut dynamic_places,
//         &depth,
//         &mut totals,
//         &treestructureformatter,
//         &mut handle,
//         &sort_type,
//     )
//     .unwrap();

//     let seconds = (start_time.elapsed()).as_secs() as f64
//         + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

//     let gigabytes = totals.size as f64 / 1_073_741_824.0;

//     println!();
//     println!("Times Processing  : {:?}s", seconds);
//     println!("Total Folders     : {}", totals.dirs);
//     println!("Total Files       : {}", totals.files);
//     println!("Total Items       : {}", totals.files + totals.dirs);
//     println!(
//         "Total Size        : {:.2} GB or {} bytes",
//         gigabytes, totals.size
//     );
//     println!();

//     Ok(())
// }
