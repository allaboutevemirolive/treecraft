use std::env;
use std::io;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

mod context;

mod flag;
use crate::flag::Flag;
use crate::flag::TreeOutput;

mod walker;
use crate::walker::WalkDir;

mod item;

mod sort;

mod error;

mod stat;
use crate::stat::head::Header;
use crate::stat::total::Totals;

mod tree;
use crate::tree::Branch;
use crate::tree::Config;
use crate::tree::Tree;

pub fn args_builder() {
    // TODO: Use Clap instead.
    let mut args: Vec<String> = env::args().collect();

    run_tree(&Flag::new(&mut args).unwrap());
}

fn run_tree(flag: &Flag) {
    // Initiate timer
    let start_time = Instant::now();

    let mut std_out = BufWriter::new(io::stdout());

    // Initialize default values for Total accumulation
    let mut total = Totals::new();

    // Set up branch configuration
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

    // Print stats based on user preference or use 'simple_indent' by default.
    TreeOutput::print_stats(&flag.tree_out, total, &mut std_out, start_time, tree.branch);
}
