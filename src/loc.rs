#[derive(Debug, PartialEq, Default)]
pub enum PrintLocation {
    /// Print in textfile
    File,
    /// Print in terminal
    #[default]
    Stdout,
}
