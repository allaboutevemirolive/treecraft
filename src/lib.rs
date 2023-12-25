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
use std::path::Path;
use std::time::Instant;

pub fn args_builder() {
    let mut args: Vec<String> = env::args().collect();

    let options = Options::new(&mut args);

    initializer(&options);
}

// TODO
pub fn initializer(options: &Options) {
    // TODO: Specify outputfile's name
    let mut handler = (options.loc).output_writer().unwrap();

    let mut totals = Totals::new();

    let start_time = Instant::now();

    // TODO: Conjoint this variable initialization
    let dir_name = &options.target_dir;
    let path = Path::new(&dir_name);

    let tree_config = TreeConfig::new(Vec::with_capacity(5_000), 1);

    // Initialize branches
    let mut tree = Tree::new(tree_config, Branch::new());

    // TODO: Conjoint
    let header = Header::new(options, &mut handler);
    header.print_header();

    let mut walker = WalkDirs::new(&mut tree, path, &mut totals, &mut handler, options);

    walker.walk_dirs();

    if options.layout_ty == Layout::All {
        totals.stats(&mut handler, start_time, tree.branch).unwrap();
    } else {
        totals.default_stat(&mut handler).unwrap();
    }
}
