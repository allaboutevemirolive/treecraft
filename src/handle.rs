use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

/// This struct supports various data sources and formats,
///
/// including `files`, `Stdout`, `TCP streams`, `memory buffers`,
///
/// `databases`, `JSON`, `HTML`, `XML`, and more.
pub struct OutputHandler {
    inner: Rc<RefCell<dyn Write>>,
}

impl OutputHandler {
    pub fn new(inner: Rc<RefCell<dyn Write>>) -> Self {
        OutputHandler { inner }
    }
}

impl Write for OutputHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.borrow_mut().flush()
    }
}
