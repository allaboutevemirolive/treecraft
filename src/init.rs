use crate::branch::TreeStructureFormatter;
use crate::config::ConfigInfo;
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

// TODO
// Sort the fields
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

    /// Recursively explore file directories
    #[inline(always)]
    pub(crate) fn walk_dirs(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries: Vec<_> = fs::read_dir(self.path)?.collect();

        sort_entries(&mut entries, &self.flags.sorttype);

        for (index, entry) in entries.iter().enumerate() {
            // Check hidden file start with '.'
            match entry.as_ref() {
                Ok(entry) => {
                    if Self::check_hidden_file(entry) {
                        self.totals.hidden_file += 1;
                        continue;
                    }
                }
                Err(err) => {
                    eprintln!("Error while retrieving hidden file (files/dirs's name start with '.') entry: {}", err);
                }
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

            // Configure the ways we collect metada
            let info = ConfigInfo::new(entry.as_ref().unwrap(), self.depth, &self.flags.config)?;

            let visitor = Visitor::new(
                info,
                self.totals,
                self.fmt,
                self.handler,
                self.nodes,
                self.depth,
                self.flags,
            );

            // FIXME
            if let Err(err) = visitor.visitor() {
                eprintln!("Invocation of the 'visitor' function failed: {}", err);
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
    pub info: ConfigInfo,
    pub totals: &'a mut Totals,
    pub fmt: &'a TreeStructureFormatter,
    pub handler: &'a mut OutputHandler,
    pub nodes: &'a mut Vec<i32>,
    pub depth: &'a i32,
    pub flags: &'a Flags,
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

    // FIXME
    // Use struct to handle different output instead of enum
    // TODO
    // Bad design?
    #[inline(always)]
    fn visitor(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.info {
            ConfigInfo::All(ref info) => {
                if let Err(err) = info.all_visitor(
                    self.flags,
                    self.handler,
                    self.totals,
                    self.nodes,
                    self.fmt,
                    self.depth,
                ) {
                    eprintln!("Invocation of the 'all_visitor' function failed: {}", err);
                }
            }
            ConfigInfo::Default(ref info) => {
                // TODO
                // Make sure it same as Visitor struct
                if let Err(err) = info.default_visitor(
                    self.flags,
                    self.handler,
                    self.totals,
                    self.nodes,
                    self.fmt,
                    self.depth,
                ) {
                    eprintln!(
                        "Invocation of the 'default_visitor' function failed: {}",
                        err
                    );
                }
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

    /// Print the name and full path of the target directory
    /// or the current dir if none is specified.
    #[inline(always)]
    pub(crate) fn print_header(self) -> Result<(), Box<dyn std::error::Error>> {
        let dir_name = Path::new(&self.flags.dir_path);
        let dir_name_os = dir_name.file_name().unwrap_or_default();
        let curr_dir = &dir_name_os.to_string_lossy();

        //
        // release
        //    .
        //    ├── build
        //    ├── deps
        //

        // TODO
        // We need to make sure the 'self.handler' is at the center of 'dot'
        // Calculate the middle of 'self.handler', then alignt it with 'dot'
        write!(self.handler, "\n {}\n    .\n", curr_dir,)?;

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
