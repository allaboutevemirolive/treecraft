use crate::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{env, io};

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

pub mod output_ty {
    pub static STD_OUT: &str = "Terminal";
    pub static FILE: &str = "File";
}

#[derive(Debug, PartialEq)]
pub enum Layout {
    Default,
    All,
}

// TODO: Add option to printout in .md, html ...
#[derive(Debug, PartialEq)]
pub enum Location {
    Stdout,
    File,
}

impl Location {
    #[rustfmt::skip]
    pub(crate) fn output_writer(&self, options: &Options ) -> Result<OutputHandler, Box<dyn std::error::Error>> {
        let output_writer: Box<dyn Write> = match self {
            Location::File => Box::new(
                BufWriter::new(File::create(&options.out_filename)?)
            ),
            Location::Stdout => Box::new(
                BufWriter::new(io::stdout().lock())
            ),
        };

        Ok(OutputHandler::new(Rc::new(RefCell::new(output_writer))))
    }
}

pub struct Options {
    pub target_dir: String,
    pub out_filename: String,
    pub sort_ty: Sort,
    pub loc: Location,
    pub layout_ty: Layout,
}

impl Options {
    pub fn new(args: &mut Vec<String>) -> Options {
        let mut default_flags: Options = Options {
            target_dir: get_absolute_current_dir(),
            out_filename: "Out.txt".to_owned(),
            sort_ty: Sort::CaseSensitive,
            loc: Location::Stdout,
            layout_ty: Layout::All,
        };

        get_target_path(args, &mut default_flags);

        process_flags(args, &mut default_flags);

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
        // TODO: We want '--file=hello.txt', default 'Out.txt'
        .arg(
            Arg::new(output_ty::FILE)
                .long("file")
                .short('f')
                .help("Printout output in a text file")
                .action(ArgAction::SetTrue),
        )
}

// Sometimes we pass 'target path' instead of 'current path'
// We need to delete those target path before pass it to 'tc_app'
fn get_target_path(args: &mut Vec<String>, default_flags: &mut Options) {
    let mut delete_index = None;

    for (index, arg) in args.iter().skip(1).enumerate() {
        if let Some(path) = valid_path(arg) {
            default_flags.target_dir = path.to_string_lossy().to_string();
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

fn process_flags(args: &mut Vec<String>, default_flags: &mut Options) {
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

    match matches.get_flag(output_ty::FILE) {
        true => {
            default_flags.loc = Location::File;
        }
        false => {}
    }
}
