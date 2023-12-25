use crate::stat::total::Totals;
use crate::WalkDirs;
use std::fs::{self, DirEntry, FileType};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::flag::Flags;
use crate::handle::loc::Location;
use crate::handle::loc::OutputHandler;
use crate::tree::Tree;
use colored::*;

/*
TODO: Implement specialize crate to collect file metada.
*/

pub struct ItemCollector {
    pub name: String,
    pub path: PathBuf,
    pub depth: usize,
    pub file_type: FileType,
    pub size: u64,
}

impl ItemCollector {
    pub fn new(entry: &DirEntry, depth: &usize) -> io::Result<ItemCollector> {
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
        tree: &mut Tree,
    ) {
        if self.file_type.is_dir() {
            self.process_dir(flags, handler, total, tree);
        } else {
            self.process_file(handler, total);
        }

        total.size += self.size;
    }

    // TODO: 'process_dir' and 'process_file' should be a trait
    fn process_dir(
        &self,
        flags: &Flags,
        handler: &mut OutputHandler,
        total: &mut Totals,
        tree: &mut Tree,
    ) {
        // TODO
        // Avoid ANSI color if printing in a file,
        // but include ANSI when printing to the terminal.
        if flags.loc == Location::File {
            writeln!(handler, "{}", &self.name).unwrap_or_default();
        } else {
            writeln!(handler, "{}", &self.name.bright_green()).unwrap_or_default();
        }

        total.directories += 1;

        tree.config.depth.0 += 1;

        // Iterate next depth of file, to perform DFS
        let mut walker = WalkDirs::new(tree, &self.path, total, handler, flags);
        walker.walk_dirs();
    }

    fn process_file(&self, handler: &mut OutputHandler, total: &mut Totals) {
        writeln!(handler, "{}", &self.name).unwrap_or_default();
        // Update file count
        total.files += 1;
    }
}
