pub mod config;
pub mod flag;
pub mod fmt;
pub mod handle;
pub mod init;
pub mod sort;
pub mod total;
use flag::Flags;
use init::initializer;
use std::env;
use std::io::{self, Write};

const HELP_TEXT: &str = "\
-tf                         Output the tree view to a text file.
-ci                         Sort filenames with case-insensitive.
-cs                         Sort filenames with case-sensitive.
-no                         Do not sort.
-xt                         Sort based on file's extension.
-help                       Display usage information and exit.";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    process_args()?;
    Ok(())
}

pub fn process_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut flags = Flags::new();
    flags.processing_args(env::args().collect());

    if flags.help {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(&mut handle, "{}", HELP_TEXT)?;
        std::process::exit(0);
    }

    initializer(&flags)?;

    Ok(())
}
