use std::env;
use std::io::{self, Write};
use treecraft::flag::Flags;
use treecraft::init::initializer;

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
    let args: Vec<String> = env::args().collect();
    let mut flags = Flags::new();
    flags.processing_args(args);

    if flags.help {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(&mut handle, "{}", HELP_TEXT)?;
        std::process::exit(0);
    }

    initializer(&flags)?;

    Ok(())
}
