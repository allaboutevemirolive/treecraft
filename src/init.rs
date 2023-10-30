use crate::branch::TreeStructureFormatter;
use crate::config::Config;
use crate::config::ConfigAll;
use crate::config::ConfigDefault;
use crate::config::ConfigInfo;
use crate::config::DisplayBrightGreen;
use crate::config::DisplayOsString;
use crate::flag::Flags;
use crate::handle::OutputHandler;
use crate::sort::sort_entries;
use crate::total::Totals;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub enum PrintLocation {
    /// Print in textfile
    File,
    /// Print in terminal
    #[default]
    Stdout,
}

pub struct WalkDirs<'a> {
    path: &'a Path,
    nodes: &'a mut Vec<i32>,
    depth: &'a i32,
    totals: &'a mut Totals,
    fmt: &'a TreeStructureFormatter,
    handler: &'a mut OutputHandler,
    flags: &'a Flags,
}

impl<'a> WalkDirs<'a> {
    #[inline(always)]
    pub(crate) fn new(
        path: &'a Path,
        nodes: &'a mut Vec<i32>,
        depth: &'a i32,
        totals: &'a mut Totals,
        fmt: &'a TreeStructureFormatter,
        handler: &'a mut OutputHandler,
        flags: &'a Flags,
    ) -> Self {
        WalkDirs {
            path,
            nodes,
            depth,
            totals,
            fmt,
            handler,
            flags,
        }
    }

    #[inline(always)]
    pub(crate) fn walk_dirs(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries: Vec<_> = fs::read_dir(self.path)?.collect();

        sort_entries(&mut entries, &self.flags.sorttype);

        for (index, entry) in entries.iter().enumerate() {
            // Check hidden file start with '.'
            if let Ok(entry) = entry.as_ref() {
                if Self::check_hidden_file(entry) {
                    self.totals.hidden_file += 1;
                    continue;
                }
            } else {
                eprintln!("Error while retrieving a dot file entry");
            }

            // Modifying the current vector for generating tree branch
            if index < entries.len() - 1 {
                self.nodes.push(1);
            } else {
                self.nodes.push(2);
            };

            // Print branch
            self.fmt
                .print_tree(self.nodes, self.nodes.len() - 1, self.handler)?;

            // Choose the way we collecting metada
            // FIXME
            // This code add unnecessary complexity.
            #[rustfmt::skip]
            let info = match self.flags.config {
                Config::All => {
                    ConfigInfo::All(
                        ConfigAll::new(entry.as_ref().unwrap(), self.depth)?
                    )
                }
                Config::Default => {
                    ConfigInfo::Default(
                        ConfigDefault::new(entry.as_ref().unwrap(), self.depth)?
                    )
                }
            };

            let visitor = Visitor::new(
                info,
                self.totals,
                self.fmt,
                self.handler,
                self.nodes,
                self.depth,
                self.flags,
            );

            if let Err(err) = visitor.visitor() {
                eprintln!("Error: {}", err);
            }

            self.nodes.pop();
        }

        Ok(())
    }

    fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
        let check_hidden = check_hidden.file_name();
        let entry_name = check_hidden.to_string_lossy();

        entry_name.starts_with('.')
    }
}

pub struct Visitor<'a> {
    info: ConfigInfo,
    totals: &'a mut Totals,
    fmt: &'a TreeStructureFormatter,
    handler: &'a mut OutputHandler,
    nodes: &'a mut Vec<i32>,
    depth: &'a i32,
    flags: &'a Flags,
}

impl<'a> Visitor<'a> {
    #[inline(always)]
    fn new(
        info: ConfigInfo,
        totals: &'a mut Totals,
        fmt: &'a TreeStructureFormatter,
        handler: &'a mut OutputHandler,
        nodes: &'a mut Vec<i32>,
        depth: &'a i32,
        flags: &'a Flags,
    ) -> Self {
        Visitor {
            info,
            totals,
            fmt,
            handler,
            nodes,
            depth,
            flags,
        }
    }

    #[inline(always)]
    fn visitor(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.info {
            ConfigInfo::All(info) => {
                if info.file_type.is_dir() {
                    // Avoid ANSI color if printing in a file,
                    // but include ANSI when printing to the terminal.
                    if self.flags.output == PrintLocation::File {
                        writeln!(self.handler, "{}", DisplayOsString(&info.name))?;
                    } else {
                        writeln!(self.handler, "{}", DisplayBrightGreen(&info.name))?;
                    }

                    self.totals.directories += 1;

                    let next_depth = self.depth + 1;

                    let walker = WalkDirs::new(
                        &info.path,
                        self.nodes,
                        &next_depth,
                        self.totals,
                        self.fmt,
                        self.handler,
                        self.flags,
                    );

                    if let Err(err) = walker.walk_dirs() {
                        eprintln!("Error: {}", err);
                    }
                } else {
                    writeln!(self.handler, "{}", DisplayOsString(&info.name))?;
                    self.totals.files += 1;
                }

                self.totals.size += info.size;
            }
            ConfigInfo::Default(info) => {
                if info.file_type.is_dir() {
                    // Avoid ANSI color if printing in a file,
                    // but include ANSI when printing to the terminal.
                    if self.flags.output == PrintLocation::File {
                        writeln!(self.handler, "{}", DisplayOsString(&info.name))?;
                    } else {
                        writeln!(self.handler, "{}", DisplayBrightGreen(&info.name))?;
                    }

                    self.totals.directories += 1;

                    let next_depth = self.depth + 1;

                    let walker = WalkDirs::new(
                        &info.path,
                        self.nodes,
                        &next_depth,
                        self.totals,
                        self.fmt,
                        self.handler,
                        self.flags,
                    );

                    if let Err(err) = walker.walk_dirs() {
                        eprintln!("Error: {}", err);
                    }
                } else {
                    writeln!(self.handler, "{}", DisplayOsString(&info.name))?;
                    self.totals.files += 1;
                }

                self.totals.size += info.size;
            }
        }

        Ok(())
    }
}

pub struct Header<'a> {
    flags: &'a Flags,
    handler: &'a mut OutputHandler,
}

impl<'a> Header<'a> {
    #[inline(always)]
    pub(crate) fn new(flags: &'a Flags, handler: &'a mut OutputHandler) -> Self {
        Header { flags, handler }
    }

    #[inline(always)]
    pub(crate) fn print_header(self) -> Result<(), Box<dyn std::error::Error>> {
        let dir_name = Path::new(&self.flags.dir_path);
        let binding = dir_name.file_name().unwrap_or_default();
        let curr_path = &binding.to_string_lossy();
        let separator = "-".repeat(curr_path.len());

        write!(
            self.handler,
            "\n{} ({})\n{} \n",
            curr_path,
            DisplayOsString(&self.flags.dir_path),
            separator
        )?;

        Ok(())
    }
}

/// Select the output type based on the provided flag.
///
/// Options include 'terminal' or 'textfile'.
pub(crate) fn output_writer(
    print_location: &PrintLocation,
) -> Result<OutputHandler, Box<dyn std::error::Error>> {
    match print_location {
        PrintLocation::File => {
            // TODO:
            // Allow the user to specify the output file.
            // If no output is defined, the default is 'Output.txt'.
            let output_file = File::create("Output.txt")?;
            let file_writer = BufWriter::new(output_file);
            #[rustfmt::skip]
            let file_writer_refcell = Rc::new(
                RefCell::new(file_writer)
            );
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
