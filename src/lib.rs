pub mod flag;
pub mod handle;
pub mod init;
pub mod item;
pub mod sort;
pub mod stat;
pub mod tree;

use crate::flag::Layout;
use stat::head::Header;

use crate::flag::*;
use crate::handle::*;
use crate::init::*;
use crate::sort::*;
use crate::stat::total::*;
use crate::tree::*;

use clap::{Arg, ArgAction, Command};
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub mod options {
    pub static GUIDE: &str = "Guide";
    pub static FILES: &str = "FILES";
}

pub mod layout {
    pub static ALL: &str = "All";
    pub static DEFAULT: &str = "Default";
}

pub mod sort_ty {
    pub static SORT_CASE_SENSITIVE: &str = "Sort_case_sensitive";
    pub static SORT_CASE_INSENSITIVE: &str = "Sort_case_insensitive";
    pub static SORT_BY_EXTENSION: &str = "Sort_by_extension";
    pub static NONE: &str = "None";
}

const HELP_TEXT: &str = "
Usage:
  treecraft [OPTIONS]

Options:
  -d, --default           Print the tree view to a text file.
  -g, --guide             Display this help message and exit.
  -s, --case-sensitive    Sort filenames in case-sensitive order.
";

pub fn arg_builder() {
    let mut args: Vec<String> = env::args().collect();

    let flags = Flags::new(&mut args);

    if flags.guide {
        let mut stdout = io::stdout().lock();
        writeln!(&mut stdout, "{}\n", HELP_TEXT).unwrap_or_default();
        std::process::exit(0);
    }

    initializer(&flags);
}

pub fn tc_app() -> Command {
    Command::new("treecraft")
        .arg(
            Arg::new(layout::DEFAULT)
                .long("default")
                .short('d')
                .help("Print output in a text file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::GUIDE)
                .long("guide")
                .short('g')
                .help("Print help message")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(sort_ty::SORT_CASE_SENSITIVE)
                .long("case-sensitive")
                .short('s')
                .help("Sort list in case-sensitive")
                .action(ArgAction::SetTrue),
        )
}

// TODO
pub fn initializer(flags: &Flags) {
    // TODO: Specify outputfile's name
    let mut handler = (flags.loc).output_writer().unwrap();

    let mut totals = Totals::new();

    let start_time = Instant::now();

    // TODO: Conjoint this variable initialization
    let dir_name = flags.target_dir.to_string_lossy().into_owned();
    let path = Path::new(&dir_name);

    let tree_config = TreeConfig::new(Vec::with_capacity(5_000), tree::TreeDepth(1));

    // Initialize branches
    let mut tree = Tree::new(tree_config, Branch::new());

    // TODO: Conjoint
    let header = Header::new(flags, &mut handler);
    header.print_header();

    let mut walker = WalkDirs::new(&mut tree, path, &mut totals, &mut handler, flags);

    walker.walk_dirs();

    if flags.layout_ty == Layout::All {
        totals.stats(&mut handler, start_time, tree.branch).unwrap();
    } else {
        totals.default_stat(&mut handler).unwrap();
    }
}
