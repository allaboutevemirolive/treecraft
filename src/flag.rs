use crate::handle::loc::Location;
use crate::*;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Default)]
pub enum Layout {
    #[default]
    Default,
    All,
}

pub struct Flags {
    pub target_dir: OsString,
    pub sort_ty: Sort,
    pub loc: Location,
    pub layout_ty: Layout,
    pub guide: bool,
}

impl Flags {
    pub fn new(args: &mut Vec<String>) -> Flags {
        let mut default_flags: Flags = Flags {
            target_dir: OsString::from(get_absolute_current_dir()),
            sort_ty: Sort::CaseSensitive,
            loc: Location::Stdout,
            layout_ty: Layout::All,
            guide: false,
        };

        // Sometimes we pass target path instead of current path
        // We need to delete those target path before pass it to 'tc_app'

        // Find the index of the argument to delete
        let mut delete_index = None;

        for (index, arg) in args.iter().skip(1).enumerate() {
            if let Some(path) = valid_path(arg) {
                default_flags.target_dir = path.into_os_string();
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

        match matches.get_flag(options::GUIDE) {
            true => {
                default_flags.guide = true;
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
