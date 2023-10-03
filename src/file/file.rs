use std::cell::RefCell;
use std::io::{self, Write};
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

/// This struct supports various data sources and formats, 
/// 
/// including `files`, `Stdout`, `TCP streams`, `memory buffers`, 
/// 
/// `databases`, `JSON`, `HTML`, `XML`, and more.
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
