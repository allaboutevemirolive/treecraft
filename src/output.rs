use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    time::Instant, rc::Rc, cell::RefCell,
};

use crate::{
    file::file::{OutputHandle, OutputType},
    flag::Flags,
    format::TreeStructureFormatter,
    read_directory_recursive,
    total::Totals,
};

// pub fn run_terminal(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
//     let directory_path = &flags.dirname.to_string();
//     let sort_type = &flags.sorttype;
//     // Main place to determine the structure of branch
//     let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
//     let depth = 1;
//     let mut totals = Totals::new();
//     let treestructureformatter = TreeStructureFormatter::new();

//     let stdout = io::stdout();
//     let mut handle = stdout.lock();

//     println!("{}", flags.dirname.to_string());
//     let start_time = Instant::now();

//     read_directory_recursive(
//         Path::new(&directory_path),
//         &mut dynamic_places,
//         &depth,
//         &mut totals,
//         &treestructureformatter,
//         &mut handle,
//         &sort_type,
//         &flags,
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

// pub fn run_text_file(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
//     let directory_path = &flags.dirname.to_string();
//     let sort_type = &flags.sorttype;
//     // Main place to determine the structure of branch
//     let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
//     let depth = 1;
//     let mut totals = Totals::new();
//     let treestructureformatter = TreeStructureFormatter::new();

//     // let mut handle = None;

//     // FIXME: Enable user to define the output_file's name and file extenstion
//     let output_file_path = "Output.txt";
//     let output_file = File::create(&output_file_path)?;
//     // Redirect stdout to the output file
//     let stdout = io::stdout();
//     let _stdout_guard = stdout.lock();
//     io::stdout().flush()?;
//     io::stdout().lock().flush()?;

//     let mut handle = io::BufWriter::new(&output_file);

//     println!("{}", flags.dirname.to_string());
//     let start_time = Instant::now();

//     read_directory_recursive(
//         Path::new(&directory_path),
//         &mut dynamic_places,
//         &depth,
//         &mut totals,
//         &treestructureformatter,
//         &mut handle,
//         &sort_type,
//         &flags,
//     )?;

//     let seconds = (start_time.elapsed()).as_secs() as f64
//         + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

//     let gigabytes = totals.size as f64 / 1_073_741_824.0;

//     writeln!(&mut handle)?;
//     writeln!(&mut handle, "Times Processing  : {:?}s", seconds)?;
//     writeln!(&mut handle, "Total Folders     : {}", totals.dirs)?;
//     writeln!(&mut handle, "Total Files       : {}", totals.files)?;
//     writeln!(
//         &mut handle,
//         "Total Items       : {}",
//         totals.files + totals.dirs
//     )?;
//     writeln!(
//         &mut handle,
//         "Total Size        : {:.2} GB or {} bytes",
//         gigabytes, totals.size
//     )?;

//     // Explicit close
//     drop(handle);
//     drop(output_file);

//     Ok(())
// }

pub fn run_main(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = &flags.dirname.to_string();
    let sort_type = &flags.sorttype;
    // Main place to determine the structure of branch
    let mut dynamic_places: Vec<i32> = Vec::with_capacity(1);
    let depth = 1;
    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();

    let mut handle: OutputHandle;

    // We handle textfile and terminal output
    match OutputType::File {
        OutputType::File => {
            let output_file = File::create("Output.txt")?;
            let file_writer = io::BufWriter::new(output_file);
            let file_writer_refcell = Rc::new(RefCell::new(file_writer));
            handle = OutputHandle::new(file_writer_refcell);
        }
        OutputType::Stdout => {
            let stdout = io::stdout();
            let stdout_writer = io::BufWriter::new(stdout.lock());
            let stdout_writer_refcell = Rc::new(RefCell::new(stdout_writer));
            handle = OutputHandle::new(stdout_writer_refcell);
        }
    }

    println!("{}", flags.dirname.to_string());
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

    match flags.output {
        OutputType::File => {
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

            // 'handle' cannot be use after this
            drop(handle);
        }
        OutputType::Stdout => {
            let seconds = (start_time.elapsed()).as_secs() as f64
                + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

            let gigabytes = totals.size as f64 / 1_073_741_824.0;

            println!();
            println!("Times Processing  : {:?}s", seconds);
            println!("Total Folders     : {}", totals.dirs);
            println!("Total Files       : {}", totals.files);
            println!("Total Items       : {}", totals.files + totals.dirs);
            println!(
                "Total Size        : {:.2} GB or {} bytes",
                gigabytes, totals.size
            );
            println!();
        }
    }

    Ok(())
}
