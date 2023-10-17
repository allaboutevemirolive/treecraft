#[derive(Debug, PartialEq)]
pub enum PrintLocation {
    /// Print in textfile
    File,
    /// Print in terminal
    Stdout,
}

impl Default for PrintLocation {
    fn default() -> Self {
        PrintLocation::Stdout
    }
}
