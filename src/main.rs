pub mod flag;
pub mod fmt;
pub mod handler;
pub mod init;
pub mod meta;
pub mod sort;
use flag::Flags;
use init::initializer;
use std::env;
use std::io::{self, Write};

const HELP_TEXT: &str = "\
-tf                         Print output in a text file
-st-fn-lc                   Sort filename with case insensitive or lowercase
-st-fn                      Sort filename
-st-no                      No sort
-help                       Print usage and exit";

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
