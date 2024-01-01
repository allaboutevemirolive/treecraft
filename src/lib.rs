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

    exec(&Options::new(&mut args));
}

// TODO
#[rustfmt::skip]
pub fn exec(options: &Options) {
    // TODO: Specify outputfile's name
    let mut handler = (options.loc).output_writer(options).unwrap();

    let mut totals = Totals::new();

    let start_time = Instant::now();

    let tree_config = TreeConfig::new(Vec::with_capacity(5_000), 1);

    // Initialize branches
    let mut tree = Tree::new(tree_config, Branch::new());

    Header::new(
        options, 
        &mut handler
    )
    .print_header();

    WalkDirs::new(
        &mut tree,
        Path::new(&options.target_dir),
        &mut totals,
        &mut handler,
        options,
    )
    .walk_dirs();

    if options.layout_ty == Layout::All {
        totals.stats(&mut handler, start_time, tree.branch).unwrap();
    } else {
        totals.default_stat(&mut handler).unwrap();
    }
}
