use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::rc::Rc;



#[derive(Debug, PartialEq)]
pub enum OutputType {
    /// Print in text file
    File,
    /// Print in terminal
    Stdout,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::Stdout
    }
}


/// This struct can handle:
/// - File
/// - Stdout
/// - TcpStream
/// - Memory Buffer
/// - Database
/// - JSON 
/// - HTML
/// - XML
/// - and others
pub struct OutputHandle {
    inner: Rc<RefCell<dyn Write>>,
}

impl OutputHandle {
    pub fn new(inner: Rc<RefCell<dyn Write>>) -> Self {
        OutputHandle { inner }
    }
}

impl Write for OutputHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.borrow_mut().flush()
    }
}

// pub fn write_output(output_type: OutputType, data: &str) -> io::Result<()> {
//     match output_type {
//         OutputType::File => {
//             let file = File::create("output.txt")?;
//             let mut file_writer = BufWriter::new(&file);
//             let mut file_output = OutputHandle::new(&mut file_writer);
//             file_output.write_all(data.as_bytes())?;
//         }
//         OutputType::Stdout => {
//             let stdout = io::stdout();
//             let mut stdout_writer = io::BufWriter::new(stdout.lock());
//             let mut stdout_output = OutputHandle::new(&mut stdout_writer);
//             stdout_output.write_all(data.as_bytes())?;
//         }
//     }
//     Ok(())
// }

// fn main() -> io::Result<()> {
//     let data = "Hello, World!";
//     let output_type = OutputType::File; // Change to the desired output type
//     write_output(output_type, data)?;
//     Ok(())
// }
