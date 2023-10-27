use crate::config::Config;
use crate::config::ConfigAll;
use crate::config::ConfigDefault;
use crate::config::ConfigInfo;
use crate::config::DisplayBrightGreen;
use crate::config::DisplayBrightYellow;
use crate::config::DisplayOsString;
use crate::flag::Flags;
use crate::fmt::TreeStructureFormatter;
use crate::handle::OutputHandler;
use crate::sort::{sort_entries, Sort};
use crate::total::Totals;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, PartialEq, Default)]
pub enum PrintLocation {
    /// Print in textfile
    File,
    /// Print in terminal
    #[default]
    Stdout,
}

// struct MyStruct<T> {
//     data: i32,
//     phantom: PhantomData<T>,
// }

// #[derive(Debug)]
// pub struct WalkDir<'a> {
//     path: &'a Path,
//     nodes: &'a mut Vec<i32>,
//     depth: &'a i32,
//     totals: &'a mut Totals,
//     fmt: &'a TreeStructureFormatter,
//     handler: &'a mut OutputHandler,
//     sorter: &'a Sort,
//     loc: &'a PrintLocation,
//     config: &'a Config,
// }

// impl<'a> WalkDir<'a> {
//     pub fn new(
//         path: &'a Path,
//         nodes: &'a mut Vec<i32>,
//         depth: &'a i32,
//         totals: &'a mut Totals,
//         fmt: &'a TreeStructureFormatter,
//         handler: &'a mut OutputHandler,
//         sorter: &'a Sort,
//         loc: &'a PrintLocation,
//         config: &'a Config,
//     ) -> Self {
//         // println!("--------------");
//         WalkDir {
//             path,
//             nodes,
//             depth,
//             totals,
//             fmt,
//             handler,
//             sorter,
//             loc,
//             config,
//         }
//     }
// }

// impl<'a> Iterator for WalkDir<'a> {
//     type Item = Result<(), Box<dyn std::error::Error>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut entries: Vec<_> = fs::read_dir(self.path).unwrap().collect();

//         println!("----------");
//         sort_entries(&mut entries, self.sorter);

//         for (index, entry) in entries.iter().enumerate() {
//             // Modifying the current vector for generating tree branch
//             if index < entries.len() - 1 {
//                 self.nodes.push(1);
//             } else {
//                 self.nodes.push(2);
//             };

//             self.fmt
//                 .print_tree(self.nodes, self.nodes.len() - 1, self.handler)
//                 .unwrap();

//             // #[rustfmt::skip]
//             let info = match self.config {
//                 Config::All => {
//                     ConfigInfo::All(ConfigAll::new(entry.as_ref().unwrap(), self.depth).unwrap())
//                 }
//                 Config::Default => ConfigInfo::Default(
//                     ConfigDefault::new(entry.as_ref().unwrap(), self.depth).unwrap(),
//                 ),
//             };

//             visitor(
//                 info,
//                 self.totals,
//                 self.fmt,
//                 self.handler,
//                 self.loc,
//                 self.nodes,
//                 self.depth,
//                 self.sorter,
//                 self.config,
//             )
//             .unwrap_or_default();

//             self.nodes.pop();
//         }
//         // unimplemented!();
//         Some(Ok(()))
//     }
// }

// TODO
// Create custom error message
#[allow(clippy::cognitive_complexity)]
#[cfg(any(unix, windows))]
pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {
    let mut totals = Totals::new();
    let mut handler = output_writer(&flags.output)?;
    let start_time = Instant::now();

    header_info(flags, &mut handler).unwrap_or_default();

    dir_info(flags, &mut handler).unwrap_or_default();

    // WalkDir::new(
    //     Path::new(&flags.dirname.to_string_lossy().into_owned()),
    //     // This is the primary location to determine the structure of the branch.
    //     &mut Vec::with_capacity(5_000),
    //     &1,
    //     &mut totals,
    //     &TreeStructureFormatter::new(),
    //     &mut handler,
    //     &flags.sorttype,
    //     &flags.output,
    //     &flags.config,
    // );

    // WalkDirs::new(
    //     Path::new(&flags.dirname.to_string_lossy().into_owned()),
    //     nodes,
    //     depth,
    //     &mut totals,
    //     fmt,
    //     &mut handler,
    //     sorter,
    //     loc,
    //     config
    // );

    let binding = flags.dirname.to_string_lossy().into_owned();
    let path = Path::new(&binding);
    let mut binding = Vec::with_capacity(5_000);
    let fmt = TreeStructureFormatter::new();

    let walker = WalkDirs::new(
        path,
        // This is the primary location to determine the structure of the branch.
        &mut binding,
        &1,
        &mut totals,
        &fmt,
        &mut handler,
        &flags.sorttype,
        &flags.output,
        &flags.config,
    );

    if let Err(err) = walker.walk_dirs() {
        eprintln!("Error: {}", err);
    }

    // WalkDirs::walk_dirs(self.path, nodes, depth, &mut totals, fmt, &mut handler, sorter, loc, config);

    // walk_dirs(
    //     Path::new(&flags.dirname.to_string_lossy().into_owned()),
    //     // This is the primary location to determine the structure of the branch.
    //     &mut Vec::with_capacity(5_000),
    //     &1,
    //     &mut totals,
    //     &TreeStructureFormatter::new(),
    //     &mut handler,
    //     &flags.sorttype,
    //     &flags.output,
    //     &flags.config,
    // )?;

    handler.flush()?;

    stats(&mut handler, &totals, start_time)?;

    Ok(())
}

pub struct WalkDirs<'a> {
    path: &'a Path,
    nodes: &'a mut Vec<i32>,
    depth: &'a i32,
    totals: &'a mut Totals,
    fmt: &'a TreeStructureFormatter,
    handler: &'a mut OutputHandler,
    sorter: &'a Sort,
    loc: &'a PrintLocation,
    config: &'a Config,
}

impl<'a> WalkDirs<'a> {
    fn new(
        path: &'a Path,
        nodes: &'a mut Vec<i32>,
        depth: &'a i32,
        totals: &'a mut Totals,
        fmt: &'a TreeStructureFormatter,
        handler: &'a mut OutputHandler,
        sorter: &'a Sort,
        loc: &'a PrintLocation,
        config: &'a Config,
    ) -> Self {
        WalkDirs {
            path,
            nodes,
            depth,
            totals,
            fmt,
            handler,
            sorter,
            loc,
            config,
        }
    }

    fn walk_dirs(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries: Vec<_> = fs::read_dir(self.path)?.collect();

        sort_entries(&mut entries, self.sorter);

        for (index, entry) in entries.iter().enumerate() {
            // Modifying the current vector for generating tree branch
            if index < entries.len() - 1 {
                self.nodes.push(1);
            } else {
                self.nodes.push(2);
            };

            self.fmt
                .print_tree(self.nodes, self.nodes.len() - 1, self.handler)?;

            #[rustfmt::skip]
            let info = match self.config {
                Config::All => ConfigInfo::All(
                    ConfigAll::new(entry.as_ref().unwrap(), self.depth)?
                ),
                Config::Default => ConfigInfo::Default(
                    ConfigDefault::new(entry.as_ref().unwrap(), self.depth)?
                ),
            };

            Self::visitor(
                info,
                self.totals,
                self.fmt,
                self.handler,
                self.loc,
                self.nodes,
                self.depth,
                self.sorter,
                self.config,
            )?;

            self.nodes.pop();
        }

        Ok(())
    }

    fn visitor(
        info: ConfigInfo,
        totals: &mut Totals,
        fmt: &TreeStructureFormatter,
        handler: &mut OutputHandler,
        loc: &PrintLocation,
        nodes: &mut Vec<i32>,
        depth: &i32,
        sorter: &Sort,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match info {
            ConfigInfo::All(info) => {
                if info.file_type.is_dir() {
                    // Avoid ANSI color if printing in a file,
                    // but include ANSI when printing to the terminal.
                    if loc == &PrintLocation::File {
                        writeln!(handler, "{}", DisplayOsString(&info.name))?;
                    } else {
                        writeln!(handler, "{}", DisplayBrightGreen(&info.name))?;
                    }

                    totals.directories += 1;

                    let next_depth = depth + 1;

                    let walker = WalkDirs::new(
                        &info.path,
                        nodes,
                        &next_depth,
                        totals,
                        fmt,
                        handler,
                        sorter,
                        loc,
                        config,
                    );

                    if let Err(err) = walker.walk_dirs() {
                        eprintln!("Error: {}", err);
                    }

                    // Self::walk_dirs(
                    //     &info.path,
                    //     nodes,
                    //     &(depth + 1),
                    //     totals,
                    //     fmt,
                    //     handler,
                    //     sorter,
                    //     loc,
                    //     config,
                    // )?;
                } else {
                    writeln!(handler, "{}", DisplayOsString(&info.name))?;
                    totals.files += 1;
                }

                totals.size += info.size;
            }
            ConfigInfo::Default(info) => {
                if info.file_type.is_dir() {
                    // Avoid ANSI color if printing in a file,
                    // but include ANSI when printing to the terminal.
                    if loc == &PrintLocation::File {
                        writeln!(handler, "{}", DisplayOsString(&info.name))?;
                    } else {
                        writeln!(handler, "{}", DisplayBrightGreen(&info.name))?;
                    }

                    totals.directories += 1;

                    let next_depth = depth + 1;

                    let walker = WalkDirs::new(
                        &info.path,
                        nodes,
                        &next_depth,
                        totals,
                        fmt,
                        handler,
                        sorter,
                        loc,
                        config,
                    );

                    if let Err(err) = walker.walk_dirs() {
                        eprintln!("Error: {}", err);
                    }

                    // Self::walk_dirs(
                    //     &info.path,
                    //     nodes,
                    //     &(depth + 1),
                    //     totals,
                    //     fmt,
                    //     handler,
                    //     sorter,
                    //     loc,
                    //     config,
                    // )?;
                } else {
                    writeln!(handler, "{}", DisplayOsString(&info.name))?;
                    totals.files += 1;
                }

                totals.size += info.size;
            }
        }

        Ok(())
    }
}

// struct MyIterator<'a> {
//     entries: &'a [&'a str],
//     nodes: &'a mut Vec<i32>,
//     config: Config,
//     fmt: &'a TreeStructureFormatter,
//     handler: &'a mut OutputHandler,
//     loc: &'a PrintLocation,
//     depth: usize,
//     sorter: &'a Sort,
//     config_info: Option<ConfigInfo>,
// }

// impl<'a> MyIterator<'a> {
//     fn new(
//         entries: &'a [&'a str],
//         nodes: &'a mut Vec<i32>,
//         config: Config,
//         fmt: &'a TreeStructureFormatter,
//         handler: &'a mut OutputHandler,
//         loc: &'a PrintLocation,
//         depth: usize,
//         sorter: &'a Sort,
//         config_info: Option<ConfigInfo>,
//     ) -> Self {
//         MyIterator {
//             entries,
//             nodes,
//             config,
//             fmt,
//             handler,
//             loc,
//             depth,
//             sorter,
//             config_info,
//         }
//     }
// }

// impl<'a> Iterator for MyIterator<'a> {
//     type Item = Result<(), Box<dyn std::error::Error>>;

//     fn next(&mut self) -> Option<Self::Item> {

//         // Modifying the current vector for generating tree branch
//         if index < entries.len() - 1 {
//             self.nodes.push(1);
//         } else {
//             self.nodes.push(2);
//         };

//         self.fmt.print_tree(self.nodes, nodes.len() - 1, handler)?;

//         #[rustfmt::skip]
//         let info = match config {
//             Config::All => ConfigInfo::All(
//                 ConfigAll::new(entry.as_ref().unwrap(), depth)?
//             ),
//             Config::Default => ConfigInfo::Default(
//                 ConfigDefault::new(entry.as_ref().unwrap(), depth)?
//             ),
//         };

//         visitor(
//             info, totals, fmt, handler, loc, nodes, depth, sorter, config,
//         )?;

//         nodes.pop();

//         Some(Ok(()))
//     }
// }

/// Traverse nested directories recursively
/// and gather information about each folder.
// #[cfg(any(unix, windows))]
// #[allow(clippy::cognitive_complexity)]
// #[inline(always)]
// fn walk_dirs(
// path: &Path,
// nodes: &mut Vec<i32>,
// depth: &i32,
// totals: &mut Totals,
// fmt: &TreeStructureFormatter,
// handler: &mut OutputHandler,
// sorter: &Sort,
// loc: &PrintLocation,
// config: &Config,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut entries: Vec<_> = fs::read_dir(path)?.collect();

//     sort_entries(&mut entries, sorter);

//     for (index, entry) in entries.iter().enumerate() {
//         // Modifying the current vector for generating tree branch
//         if index < entries.len() - 1 {
//             nodes.push(1);
//         } else {
//             nodes.push(2);
//         };

//         fmt.print_tree(nodes, nodes.len() - 1, handler)?;

//         #[rustfmt::skip]
//         let info = match config {
//             Config::All => ConfigInfo::All(
//                 ConfigAll::new(entry.as_ref().unwrap(), depth)?
//             ),
//             Config::Default => ConfigInfo::Default(
//                 ConfigDefault::new(entry.as_ref().unwrap(), depth)?
//             ),
//         };

//         visitor(
//             info, totals, fmt, handler, loc, nodes, depth, sorter, config,
//         )?;

//         nodes.pop();
//     }

//     Ok(())
// }

// #[allow(clippy::cognitive_complexity)]
// #[inline(always)]
// fn visitor(
//     info: ConfigInfo,
//     totals: &mut Totals,
//     fmt: &TreeStructureFormatter,
//     handler: &mut OutputHandler,
//     loc: &PrintLocation,
//     nodes: &mut Vec<i32>,
//     depth: &i32,
//     sorter: &Sort,
//     config: &Config,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     match info {
//         ConfigInfo::All(info) => {
//             if info.file_type.is_dir() {
//                 // Avoid ANSI color if printing in a file,
//                 // but include ANSI when printing to the terminal.
//                 if loc == &PrintLocation::File {
//                     writeln!(handler, "{}", DisplayOsString(&info.name))?;
//                 } else {
//                     writeln!(handler, "{}", DisplayBrightGreen(&info.name))?;
//                 }

//                 totals.directories += 1;
//                 walk_dirs(
//                     &info.path,
//                     nodes,
//                     &(depth + 1),
//                     totals,
//                     fmt,
//                     handler,
//                     sorter,
//                     loc,
//                     config,
//                 )?;
//             } else {
//                 writeln!(handler, "{}", DisplayOsString(&info.name))?;
//                 totals.files += 1;
//             }

//             totals.size += info.size;
//         }
//         ConfigInfo::Default(info) => {
//             if info.file_type.is_dir() {
//                 // Avoid ANSI color if printing in a file,
//                 // but include ANSI when printing to the terminal.
//                 if loc == &PrintLocation::File {
//                     writeln!(handler, "{}", DisplayOsString(&info.name))?;
//                 } else {
//                     writeln!(handler, "{}", DisplayBrightGreen(&info.name))?;
//                 }

//                 totals.directories += 1;
//                 walk_dirs(
//                     &info.path,
//                     nodes,
//                     &(depth + 1),
//                     totals,
//                     fmt,
//                     handler,
//                     sorter,
//                     loc,
//                     config,
//                 )?;
//             } else {
//                 writeln!(handler, "{}", DisplayOsString(&info.name))?;
//                 totals.files += 1;
//             }

//             totals.size += info.size;
//         }
//     }

//     Ok(())
// }

/// Print-out current and target directory
fn header_info(
    flags: &Flags,
    handler: &mut OutputHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    match &flags.output {
        PrintLocation::File => {
            write!(
                handler,
                "\nTarget Path: {}\n\n",
                DisplayOsString(&flags.dirname),
            )?;
        }
        PrintLocation::Stdout => {
            write!(
                handler,
                "\nTarget Path: {}\n\n",
                DisplayBrightYellow(&flags.dirname),
            )?;
        }
    }
    Ok(())
}

/// Print-out dir's name and separator
fn dir_info(flags: &Flags, handler: &mut OutputHandler) -> Result<(), Box<dyn std::error::Error>> {
    let dir_name = Path::new(&flags.dirname);
    let binding = dir_name.file_name().unwrap_or_default();
    let curr_path = &binding.to_string_lossy();
    let separator = "-".repeat(curr_path.len());

    write!(handler, "{}\n{} \n", curr_path, separator)?;

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

fn stats(
    handler: &mut OutputHandler,
    totals: &Totals,
    start_time: Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    let seconds = (start_time.elapsed()).as_secs() as f64
        + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

    let gigabytes = totals.size as f64 / 1_073_741_824.0;

    writeln!(handler)?;
    writeln!(handler, "\nStatistics:")?;
    writeln!(handler, "-----------")?;
    writeln!(handler, "Processing Time   : {:?} seconds", seconds)?;
    writeln!(handler, "Total Directories : {}", totals.directories)?;
    writeln!(handler, "Total Files       : {}", totals.files)?;
    writeln!(
        handler,
        "Total Items       : {}",
        totals.files + totals.directories
    )?;
    writeln!(
        handler,
        "Total Size        : {:.2} GB ({} bytes)",
        gigabytes,
        format_with_commas(totals.size)
    )?;
    writeln!(handler)?;

    Ok(())
}

fn format_with_commas(num: u64) -> String {
    let mut formatted = String::new();
    let num_str = num.to_string();

    for (i, c) in num_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted.insert(0, ',');
        }
        formatted.insert(0, c);
    }

    formatted
}
