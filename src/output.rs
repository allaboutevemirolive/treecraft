use std::{
    io::{self, StdoutLock},
    path::Path,
    time::Instant, fs::File,
};

use crate::{
    meta::ext::*, 
    flag::Flags, 
    format::TreeStructureFormatter,
    read_directory_recursive, file::{io::{drop_handle_and_output_file, redirect_stdout_to_file}, file::*},
};

use crate::meta::total::*;

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Terminal,
    TextFile,
}

// enum OutputHandle {
//     TextFile(io::BufWriter<File>),
//     Terminal(StdoutLock<'static>),
//     // Add more handle types as needed
// }

impl Default for OutputType {
    fn default() -> Self {
        OutputType::Terminal
    }
}

pub fn run_terminal(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    // HardCode for debugging purpose
    // let directory_path = "/home/nemesis/Documents/Github/Focus/lang";
    // let sort_type = SortType::ByLowerCaseFileName;

    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;
    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();


    let mut handle = None;




    // if flags.output == OutputType::Terminal {
    //     let stdout = io::stdout();
    //     handle = Some(stdout.lock());
    // } 


    match flags.output {
        OutputType::Terminal => {
            let stdout = io::stdout();
            handle = Some(OutputHandle::Terminal(stdout.lock()));
        }
        OutputType::TextFile => {
            let output_file_path = "Output.txt";
            handle = Some(OutputHandle::TextFile(OutputHandler::new(output_file_path)?));
        }
    }


    


    


    println!("{}", flags.dirname.to_string());
    let mut extensions = Extensions::new();
    let start_time = Instant::now();

    read_directory_recursive(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut handle, // FIXME
        &sort_type,
        &flags,
        &mut extensions,
    )
    .unwrap();







    // Need to extract this to new file
    if flags.extensions == true {
        extensions.print_sorted_table();
    }

    println!();
    // FIXME
    println!("Times Processing  : {:?}s", calculate_elapsed_seconds(start_time));
    println!("{}", totals);
    println!();


    Ok(())
}




use std::io::Write;

pub fn run_text_file(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();
    let mut output_handler: Option<OutputHandle> = None;

    if flags.output == OutputType::TextFile {
        let output_file_path = "Output.txt";
        output_handler = Some(OutputHandle::TextFile(OutputHandler::new(output_file_path).unwrap()));
    }

    println!("{}", flags.dirname.to_string());
    let mut extensions = Extensions::new();
    let start_time = Instant::now();

    read_directory_recursive(
        Path::new(&directory_path),
        &mut dynamic_places,
        &depth,
        &mut totals,
        &treestructureformatter,
        &mut output_handler, // Pass the output handler as an Option
        &sort_type,
        &flags,
        &mut extensions,
    )?;

    if let Some(output_handler) = &mut output_handler {
        // Printing output in text file
        writeln!(output_handler, "{}", DisplayData::new(start_time, &totals))?;

        // Ensure data written immediately
        output_handler.flush()?;

        // Explicit close
        drop(output_handler);
    }

    Ok(())
}
