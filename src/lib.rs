use clap::{Arg, ArgAction, Command};
use std::env;
use std::io;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

pub mod flag;
use crate::flag::*;

pub mod walker;
use crate::walker::*;

pub mod item;

pub mod sort;
use crate::sort::*;

pub mod stat;
use crate::stat::{head::Header, total::*};

pub mod tree;
use crate::tree::*;

pub fn args_builder() {
    // TODO: Use Clap instead.
    // Collect arguments. 
    let mut args: Vec<String> = env::args().collect();

    run_tree(&Flag::new(&mut args));
}

fn run_tree(flag: &Flag) {
    // Our timer
    let start_time = Instant::now();

    let mut std_out = BufWriter::new(io::stdout());

    // Initialize default values for Total
    let mut total = Totals::new();

    // Set up branch configuration
    // The 'vector!' is the core of 'treecraft'.
    // It allows us to accurately navigate through intricate branches, 
    // even in deeply nested and complex folder structures typical of 
    // Java projects (src/main/java/smoketest/xml...).
    // Initializing the vector with a capacity of 5,000 is based on the 
    // assumption that the depth of most folders won't exceed this limit. 
    // Constantly expanding and contracting the capacity could impact runtime performance.
    let config = Config::new(Vec::with_capacity(5_000), 1);

    // Initialize tree information
    let mut tree = Tree::new(config, Branch::new());

    // Print header of our tree
    Header::new(flag, &mut std_out).print_header();

    // Iterate branches
    WalkDir::new(
        &mut tree,
        Path::new(&flag.target_dir),
        &mut total,
        &mut std_out,
        flag,
    )
    .walk();

    // Printout stats
    if flag.layout_ty == Layout::All {
        total.stats(&mut std_out, start_time, tree.branch).unwrap();
    } else {
        total.default_stat(&mut std_out).unwrap();
    }
}
