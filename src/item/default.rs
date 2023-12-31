
use crate::stat::total::Totals;
use crate::{WalkDirs};
use std::fs::{self, DirEntry, FileType};
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::flag::Options;
use crate::handle::OutputHandler;
use crate::tree::Tree;


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
    #[inline(always)]
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

    #[inline(always)]
    pub fn get_item(
        &self,
        opts: &Options,
        handler: &mut OutputHandler,
        total: &mut Totals,
        tree: &mut Tree,
    ) {
        if self.file_type.is_dir() {
            self.process_dir(opts, handler, total, tree);
        } else {
            self.process_file(handler, total);
        }

        total.size += self.size;
    }

    #[inline(always)]
    // #[rustfmt::skip]
    // TODO: 'process_dir' and 'process_file' should be a trait
    fn process_dir(
        &self,
        opts: &Options,
        handler: &mut OutputHandler,
        total: &mut Totals,
        tree: &mut Tree,
    ) {

        writeln!( handler, "{}{}{}", opts.ansi_co.bright_green, &self.name, opts.ansi_co.reset_ansi ).unwrap_or_default();

        total.directories += 1;
        tree.config.depth += 1;

        // Iterate next depth of file, to perform DFS
        WalkDirs::new(
            tree, 
            &self.path, 
            total, 
            handler, opts
        )
        .walk_dirs();
    }

    #[inline(always)]
    fn process_file(&self, handler: &mut OutputHandler, total: &mut Totals) {
        writeln!(handler, "{}", &self.name).unwrap_or_default();
        total.files += 1;
    }
}
