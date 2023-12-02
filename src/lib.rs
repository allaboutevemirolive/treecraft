pub mod flag;
pub mod handle;
pub mod init;
pub mod item;
pub mod loc;
pub mod sort;
pub mod stat;
pub mod tree;

use stat::head::Header;

use crate::flag::*;
use crate::handle::*;
use crate::init::*;
use crate::loc::*;
use crate::sort::*;
use crate::stat::total::*;
use crate::tree::*;

use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

const HELP_TEXT: &str = "
-out                        Output the tree view to a text file.
-ci                         Sort filenames with case-insensitive.
-cs                         Sort filenames with case-sensitive.
-no                         Do not sort.
-xt                         Sort based on file's extension.
-help                       Display usage information and exit.\n";

pub fn arg_builder() {
    let mut flags = Flags::new();

    flags.processing_args(env::args().collect());

    if flags.help {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(&mut handle, "{}", HELP_TEXT).unwrap_or_default();
        std::process::exit(0);
    }

    initializer(&flags);
}

pub fn initializer(flags: &Flags) {
    let mut handler = (flags.loc).output_writer().unwrap();

    let mut totals = Totals::new();
    let header = Header::new(flags, &mut handler);

    let start_time = Instant::now();

    header.print_header();

    let dir_name = flags.dir_path.to_string_lossy().into_owned();
    let path = Path::new(&dir_name);

    let mut tree = Tree::new(Vec::with_capacity(5_000), 1, Branch::new());

    let mut walker = WalkDirs::new(&mut tree, path, &mut totals, &mut handler, flags);

    walker.walk_dirs();

    totals.stats(&mut handler, start_time, tree.branch).unwrap();
}
