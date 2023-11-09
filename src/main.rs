pub mod branch;
pub mod config;
pub mod flag;
pub mod handle;
pub mod init;
pub mod sort;
pub mod total;

use crate::branch::TreeFormatter;
use crate::init::output_writer;
use crate::init::Header;
use crate::init::WalkDirs;
use crate::total::Totals;

use flag::Flags;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

const HELP_TEXT: &str = "
-out                        Output the tree view to a text file.
-ci                         Sort filenames with case-insensitive.
-cs                         Sort filenames with case-sensitive.
-no                         Do not sort.
-xt                         Sort based on file's extension.
-help                       Display usage information and exit.\n";

// In Java, when we develop software using JavaFX, we need to
// distinguish the main function in an isolated file from the
// rest of the functions to avoid undefined behavior or errors.
// Generally, it is a good practice.
//
// REF
// https://www.reddit.com/r/JavaFX/comments/k7aa9q/javafx_error_error_javafx_runtime_components_are/
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

    // TODO
    // Create custom error messages

    // INFO
    // Closure may not be ideal if the `initializer` function is intended
    // to be reused elsewhere. In such cases, it might be better
    // to keep the separate function and call it from both
    // `process_args` and other places where it's needed.
    //
    // INFO
    // Desugared version of closure:
    // pub fn initializer(flags: &Flags) -> Result<(), Box<dyn std::error::Error>> {}
    let initializer = |flags: &Flags| -> Result<(), Box<dyn std::error::Error>> {
        let mut totals = Totals::new();
        let mut handler = output_writer(&flags.output)?;
        let start_time = Instant::now();

        let header = Header::new(flags, &mut handler);

        if let Err(err) = header.print_header() {
            eprintln!("{}", err);
        }

        // INFO
        // We instantiate all the variables below in this scope
        // instead of 'WalkDirs::new', to address the short lifetime.

        let dir_name = flags.dir_path.to_string_lossy().into_owned();
        let path = Path::new(&dir_name);
        // TODO
        // Make documentation how this vecto works.
        // Primary location to determine the structure of the branch.
        let mut nodes = Vec::with_capacity(5_000);
        let tree_formatter = TreeFormatter::new();

        let walker = WalkDirs::new(
            path,
            &mut nodes,
            &1,
            &mut totals,
            &tree_formatter,
            &mut handler,
            flags,
        );

        if let Err(err) = walker.walk_dirs() {
            eprintln!("Error while walking directories: {}", err);
            std::process::exit(0);
        }

        if let Err(err) = handler.flush() {
            eprintln!("Error while flushing data: {}", err);
            std::process::exit(0);
        }

        if let Err(err) = totals.stats(&mut handler, start_time, tree_formatter) {
            eprintln!("Error while calculating statistics: {}", err);
            std::process::exit(0);
        }

        Ok(())
    };

    initializer(&flags)?;

    Ok(())
}
