use clap::{Arg, ArgAction, Command};
use std::env;
use std::path::Path;
use std::time::Instant;

pub mod flag;
use crate::flag::*;

pub mod handle;
use crate::handle::*;

pub mod init;
use crate::init::*;

pub mod item;

pub mod sort;
use crate::sort::*;

pub mod stat;
use crate::stat::{head::Header, total::*};

pub mod tree;
use crate::tree::*;

pub fn args_builder() {
    let mut args: Vec<String> = env::args().collect();

    run_tree(&Flags::new(&mut args));
}

// TODO
pub fn run_tree(flags: &Flags) {
    // TODO: Specify outputfile's name
    let mut handler = (flags.loc).output_writer(flags).unwrap();

    let mut totals = Totals::new();

    let start_time = Instant::now();

    let tree_config = TreeConfig::new(Vec::with_capacity(5_000), 1);

    // Initialize branches
    let mut tree = Tree::new(tree_config, Branch::new());

    Header::new(flags, &mut handler).print_header();

    WalkDirs::new(
        &mut tree,
        Path::new(&flags.target_dir),
        &mut totals,
        &mut handler,
        flags,
    )
    .walk_dirs();

    if flags.layout_ty == Layout::All {
        totals.stats(&mut handler, start_time, tree.branch).unwrap();
    } else {
        totals.default_stat(&mut handler).unwrap();
    }
}
