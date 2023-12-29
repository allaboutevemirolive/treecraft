use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;

/*
We may need to replace 'OutputHandler' with normal output
and rely with pipeline to procude desired output.
*/
pub struct OutputHandler {
    handle: Rc<RefCell<dyn Write>>,
}

impl fmt::Debug for OutputHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OutputHandler {{ inner: ... }}")
    }
}

impl OutputHandler {
    pub fn new(handle: Rc<RefCell<dyn Write>>) -> Self {
        Self { handle }
    }
}

impl Write for OutputHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.handle.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.handle.borrow_mut().flush()
    }
}
