use crate::PrintLocation;
use crate::Sort;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct Flags {
    pub dir_path: OsString,
    pub sort_ty: Sort,
    pub loc: PrintLocation,
    pub help: bool,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            dir_path: OsString::from(get_absolute_current_dir()),
            sort_ty: Sort::CaseSensitive,
            loc: PrintLocation::Stdout,
            help: false,
        }
    }
}

impl Flags {
    pub fn new() -> Flags {
        Default::default()
    }

    pub fn processing_args(&mut self, args: Vec<String>) {
        let mut iter = args.iter().skip(1);

        for arg in &mut iter {
            if let Some(path) = valid_path(arg) {
                self.dir_path = path.into_os_string();
            } else {
                match arg.as_str() {
                    // Output options
                    "-out" => self.loc = PrintLocation::File,

                    // Sort
                    "-def" => self.sort_ty = Sort::default(),
                    "-cs" => self.sort_ty = Sort::CaseSensitive,
                    "-ci" => self.sort_ty = Sort::CaseInsensitive,
                    "-no" => self.sort_ty = Sort::None,
                    "-xt" => self.sort_ty = Sort::Extension,

                    // Miscellaneous options
                    "-help" => self.help = true,

                    _ => {
                        break;
                    }
                }
            }
        }
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
