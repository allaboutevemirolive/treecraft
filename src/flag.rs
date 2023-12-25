use crate::handle::Location;
use crate::*;
use std::env;
use std::path::{Path, PathBuf};

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

#[derive(Debug, PartialEq)]
pub enum Layout {
    Default,
    All,
}

pub struct Options {
    pub target_dir: String,
    pub sort_ty: Sort,
    pub loc: Location,
    pub layout_ty: Layout,
}

impl Options {
    pub fn new(args: &mut Vec<String>) -> Options {
        let mut default_flags: Options = Options {
            target_dir: get_absolute_current_dir(),
            sort_ty: Sort::CaseSensitive,
            loc: Location::Stdout,
            layout_ty: Layout::All,
        };

        // Sometimes we pass 'target path' instead of 'current path'
        // We need to delete those target path before pass it to 'tc_app'

        // Find the index of the argument to delete
        let mut delete_index = None;

        for (index, arg) in args.iter().skip(1).enumerate() {
            if let Some(path) = valid_path(arg) {
                default_flags.target_dir = path.to_string_lossy().to_string();
                delete_index = Some(index + 1); // Adjust index to account for skipping the first element
                break; // Exit loop since we found a valid path
            }
        }

        // Delete argument if found
        if let Some(index) = delete_index {
            args.remove(index);
        }

        // Clone args vector before passing it to 'try_get_matches_from'
        let cloned_args: Vec<String> = args.clone();

        let matches = tc_app()
            .try_get_matches_from(cloned_args)
            .unwrap_or_else(|e| e.exit());

        match matches.get_flag(layout::DEFAULT) {
            true => {
                default_flags.layout_ty = Layout::Default;
            }
            false => {}
        }

        default_flags
    }
}

fn valid_path(arg: &str) -> Option<PathBuf> {
    let path = Path::new(arg);
    if path.is_dir() || path.is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

pub fn get_absolute_current_dir() -> String {
    env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert path to str")
        .to_string()
}

pub fn tc_app() -> Command {
    Command::new("treecraft")
        .arg(
            Arg::new(layout::DEFAULT)
                .long("default")
                .short('d')
                .help("Print default layout. Etc. GNU tree's layout")
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
