use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::rc::Rc;

/*
We may need to replace 'OutputHandler' with normal output
and rely with pipeline to procude desired output.
*/
pub struct OutputHandler {
    container: Rc<RefCell<dyn Write>>,
}

impl fmt::Debug for OutputHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OutputHandler {{ inner: ... }}")
    }
}

impl OutputHandler {
    pub fn new(container: Rc<RefCell<dyn Write>>) -> Self {
        Self { container }
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

#[derive(Debug, PartialEq, Default)]
pub enum Location {
    File,
    #[default]
    Stdout,
}

impl Location {
    pub(crate) fn output_writer(&self) -> Result<OutputHandler, Box<dyn std::error::Error>> {
        let output_writer: Box<dyn Write> = match self {
            Location::File => Box::new(BufWriter::new(File::create("Output.txt")?)),
            Location::Stdout => Box::new(BufWriter::new(io::stdout().lock())),
        };

        Ok(OutputHandler::new(Rc::new(RefCell::new(output_writer))))
    }
}
