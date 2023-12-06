use crate::item::*;
use crate::stat::total::Totals;
use crate::WalkDirs;
use std::fs::{self, DirEntry, FileType};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::flag::Flags;
use crate::handle::OutputHandler;
use crate::loc::PrintLocation;
use crate::tree::Tree;

pub struct ItemCollector {
    pub name: String,
    pub path: PathBuf,
    pub depth: u32,
    pub file_type: FileType,
    pub size: u64,
}

impl ItemCollector {
    pub fn new(entry: &DirEntry, depth: &u32) -> io::Result<ItemCollector> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        Ok(ItemCollector {
            name: full_path
                .file_name()
                .and_then(|os_str| os_str.to_str())
                .map(ToString::to_string)
                .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            size: metadata.len(),
        })
    }

    pub fn get_item(
        &self,
        flags: &Flags,
        handler: &mut OutputHandler,
        total: &mut Totals,
        mut tree: Tree,
    ) {
        if self.file_type.is_dir() {
            // Avoid ANSI color if printing in a file,
            // but include ANSI when printing to the terminal.
            if flags.loc == PrintLocation::File {
                writeln!(
                    handler,
                    "{}",
                    DisplayFormatted {
                        content: &self.name,
                        format_fn: format_default_ref_string,
                    }
                )
                .unwrap_or_default();
            } else {
                writeln!(
                    handler,
                    "{}",
                    DisplayFormatted {
                        content: &self.name,
                        format_fn: format_bright_green_ref_string,
                    }
                )
                .unwrap_or_default();
            }

            total.directories += 1;

            tree.reach += 1;

            let mut walker = WalkDirs::new(&mut tree, &self.path, total, handler, flags);

            walker.walk_dirs();
        } else {
            writeln!(
                handler,
                "{}",
                DisplayFormatted {
                    content: &self.name,
                    format_fn: format_default_ref_string,
                }
            )
            .unwrap_or_default();
            total.files += 1;
        }

        total.size += self.size;
    }
}
