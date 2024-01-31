use crate::*;

use std::env;
use std::path::{Path, PathBuf};

// How the output should looks like
// If use passed '-d' then the options below is not valid
pub mod tree_out {
    pub static VERBOSE_INDENT: &str = "Verbose_indentation";
    pub static SIMPLE_INDENT: &str = "Simple_indentation";
}

pub mod sort_ty {
    pub static SORT_CASE_SENSITIVE: &str = "Sort_case_sensitive";
    pub static SORT_CASE_INSENSITIVE: &str = "Sort_case_insensitive";
    pub static SORT_BY_EXTENSION: &str = "Sort_by_extension";
    pub static NONE: &str = "None";
}

pub mod ansi {
    pub static SHOW_ANSI: &str = "Show ansi in the output";
    pub static NO_ANSI: &str = "Exclude ansi in the output";
}

pub mod hidden_file {
    pub static SHOW: &str = "Include_hidden_file";
    pub static HIDE: &str = "Exclude_hidden_file";
}

pub mod show_path {
    pub static SHOW: &str = "Show_Path";
    pub static HIDE: &str = "HIDE_Path";
}

pub mod level {
    pub static GET: &str = "LEVEL";
}

/// Represent on how we construct the tree
#[derive(Debug, PartialEq)]
pub enum TreeOutput {
    /// By default, print simple stat
    SimpleIndent,
    // If user want default output like GNU tree
    // we print like GNU tree, etc: "tc -d"
    SimpleNoIndent,
    // Print all stat
    VerboseIndent,
}

// TODO: Add option to printout in .md, html ...
#[derive(Debug, PartialEq)]
pub enum Location {
    Stdout,
    File,
}

pub struct Flag {
    pub target_dir: String,
    pub sort_ty: Sort,
    pub loc: Location,
    pub tree_out: TreeOutput,
    pub ansi_co: AnsiColor,
    /// `True` : Show hidden files.
    ///
    /// `False` : Exclude hidden files.
    pub hidden_file: bool,
    pub show_path: bool,
    // TODO: Rename variable
    pub depth: Level,
}

pub struct AnsiColor {
    pub bright_green: String,
    pub reset_ansi: String,
}

impl Flag {
    #[allow(unused_mut)]
    pub fn new(args: &mut Vec<String>) -> Flag {
        let ansi_co = AnsiColor {
            bright_green: "\x1B[92m".to_string(),
            reset_ansi: "\x1B[0m".to_string(),
        };

        let mut lb = Level { limit: 5000 };

        let mut flag: Flag = Flag {
            target_dir: get_absolute_current_dir(),
            sort_ty: Sort::CaseSensitive,
            loc: Location::Stdout,
            tree_out: TreeOutput::SimpleIndent,
            ansi_co,
            hidden_file: false,
            show_path: false,
            depth: lb,
        };

        // TODO: Check if clap already have function to
        // retrieve path
        check_target_path(args, &mut flag);

        // TODO: Bad design?
        check_flags(args, &mut flag);

        flag
    }
}

/// Check if the path given pointed to dir or file
fn valid_path(arg: &str) -> Option<PathBuf> {
    let path = Path::new(arg);
    if path.is_dir() || path.is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

/// If no path where given, retrieve current path
/// where shell executed
pub fn get_absolute_current_dir() -> String {
    env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert path to str")
        .to_string()
}

pub fn tc_app() -> Command {
    Command::new("treecraft")
        // If user want GNU tree layout, we give simple stat but append to right
        .arg(
            Arg::new(tree_out::SIMPLE_INDENT)
                .long("default")
                .short('d')
                .help("Print default layout. Etc. GNU tree's layout")
                .action(ArgAction::SetTrue),
        )
        // If user want all stat
        .arg(
            Arg::new(tree_out::VERBOSE_INDENT)
                .long("all")
                .short('a')
                .help("Print verbose stats")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(sort_ty::SORT_CASE_SENSITIVE)
                .long("case-sensitive")
                .short('s')
                .help("Sort list in case-sensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(ansi::SHOW_ANSI)
                .long("no-ansi")
                .short('n')
                .help("Exclude ansi in the output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(hidden_file::SHOW)
                .long("hidden-file")
                .help("Include hidden file in the output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(show_path::SHOW)
                .long("show-path")
                .short('p')
                .help("Show path in the output for each directories and files")
                .action(ArgAction::SetTrue),
        )
        // DEBUG: If we pass "tc -l2 -a", the stats still 'simple_indent'.
        // We want 'verbose_indent' instead
        .arg(
            Arg::new(level::GET)
                .long("level")
                .short('l')
                .long("level")
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Set)
                .num_args(1)
                .help("Print tree until certain depth. Default depth: 5000"),
        )
}

// TODO:
#[allow(unused_mut)]
pub struct Level {
    pub limit: usize,
}

// TODO: This is temporary hack. We need to use clap instead.
// Sometimes we pass 'target path' instead of 'current path'
// We need to delete those target path before pass it to 'tc_app'
fn check_target_path(args: &mut Vec<String>, flag: &mut Flag) {
    let mut delete_index = None;

    for (index, arg) in args.iter().skip(1).enumerate() {
        if let Some(path) = valid_path(arg) {
            flag.target_dir = path.to_string_lossy().to_string();
            // Adjust index to account for skipping the first element
            delete_index = Some(index + 1);
            // Exit loop since we found a valid path
            break;
        }
    }

    if let Some(index) = delete_index {
        args.remove(index);
    }
}

fn check_flags(args: &mut [String], flag: &mut Flag) {
    let cloned_args: Vec<String> = args.to_owned();

    let matches = tc_app()
        .try_get_matches_from(cloned_args.clone())
        .unwrap_or_else(|e| e.exit());

    // Get flag given by user
    if matches.get_flag(tree_out::SIMPLE_INDENT) {
        // TODO: Confusing between 'SimpleIndent' & 'SimpleNoIndent'
        flag.tree_out = TreeOutput::SimpleNoIndent;
    } else if matches.get_flag(ansi::SHOW_ANSI) {
        flag.loc = Location::File;
        flag.ansi_co.bright_green = "".to_string();
        flag.ansi_co.reset_ansi = "".to_string();
    } else if matches.get_flag(hidden_file::SHOW) {
        flag.hidden_file = true;
    } else if matches.get_flag(show_path::SHOW) {
        flag.show_path = true;
    } else if matches.contains_id(level::GET) {
        let level: usize = *matches.get_one(level::GET).expect("default");

        flag.depth.limit = level;
    } else if matches.get_flag(tree_out::VERBOSE_INDENT) {
        flag.tree_out = TreeOutput::VerboseIndent;
    }
}
