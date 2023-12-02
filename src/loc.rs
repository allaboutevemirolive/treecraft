use crate::OutputHandler;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufWriter};
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub enum PrintLocation {
    /// Print in textfile
    File,
    /// Print in terminal
    #[default]
    Stdout,
}

impl PrintLocation {
    pub(crate) fn output_writer(&self) -> Result<OutputHandler, Box<dyn std::error::Error>> {
        match &self {
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
}
