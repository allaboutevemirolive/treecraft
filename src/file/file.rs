use std::io::{self, Write, StdoutLock, BufWriter};
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::str::Utf8Error;

use crate::file::*;

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Terminal,
    TextFile,
    // Add more output types as needed
}

pub enum OutputHandle {
    Terminal(StdoutLock<'static>),
    TextFile(file::file::OutputHandler),
}

impl OutputHandle {
    pub fn write(&mut self, data: &str) -> io::Result<()> {
        match self {
            OutputHandle::Terminal(writer) => write!(writer, "{}", data),
            OutputHandle::TextFile(writer) => write!(writer, "{}", data),
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputHandle::Terminal(writer) => writer.flush(),
            OutputHandle::TextFile(writer) => writer.flush(),
        }
    }
}

pub struct OutputHandler {
    pub handle: io::BufWriter<File>,
}

impl OutputHandler {
    pub fn new(output_file_path: &str) -> io::Result<Self> {
        let output_file = File::create(output_file_path)?;
        let handle = io::BufWriter::new(output_file);
        Ok(OutputHandler { handle })
    }

    pub fn write(&mut self, data: &str) -> io::Result<()> {
        writeln!(&mut self.handle, "{}", data)?;
        self.handle.flush()?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.handle.flush()
    }
}

impl Write for OutputHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            OutputHandle::TextFile(output_handler) => {
                let data = match std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
                    }
                };
                output_handler.write(data)?;
                Ok(buf.len())
            }
            OutputHandle::Terminal(stdout) => stdout.write(buf),
            // Handle additional cases for other handle types
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputHandle::TextFile(output_handler) => output_handler.flush(),
            OutputHandle::Terminal(stdout) => stdout.flush(),
            // Handle additional cases for other handle types
        }
    }
}











// use std::fs::File;
// use std::io::{self, Write, BufWriter, StdoutLock};


// pub struct OutputHandler<'a> {
    
//     /// This field is optional
//     /// 
//     /// Print in `TEXT FILE`
//     pub handle: BufWriter<File>,
//     /// Main default.
//     /// 
//     /// Print to `terminal`
//     pub stdout_lock: StdoutLock<'a>,
// }

// impl OutputHandler<'_> {
//     pub fn new(output_file_path: &str) -> io::Result<Self> {
//         let output_file = File::create(output_file_path)?;
//         let handle = BufWriter::new(output_file);
//         let stdout_lock = io::stdout().lock();
//         Ok(OutputHandler { handle, stdout_lock })
//     }

//     pub fn write(&mut self, data: &str) -> io::Result<()> {
//         writeln!(&mut self.handle, "{}", data)
//     }

//     pub fn flush(&mut self) -> io::Result<()> {
//         self.handle.flush()
//     }
// }


// fn main() -> io::Result<()> {
//     let output_file_path = "Output.txt";
//     let mut output_handler = OutputHandler::new(output_file_path)?;

//     // Write data to the output file
//     output_handler.write("Hello, World!")?;
    
//     // Flush to ensure data is written immediately
//     output_handler.flush()?;
    
//     // Rest of the main function
//     // ...

//     Ok(())
// }
