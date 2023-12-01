pub fn handle() {
    println!("This is handle!");
}

use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;

// #[derive(Clone)]
pub struct OutputHandler {
    container: Rc<RefCell<dyn Write>>,
}

impl fmt::Debug for OutputHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OutputHandler {{ inner: ... }}")
    }
}

impl OutputHandler {
    pub fn new(container: Rc<RefCell<dyn Write>>) -> OutputHandler {
        OutputHandler { container }
    }
}

impl Write for OutputHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.container.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.container.borrow_mut().flush()
    }
}
