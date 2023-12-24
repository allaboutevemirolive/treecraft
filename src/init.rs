use std::{fs, path::Path};

use crate::flag::Flags;
use crate::handle::loc::OutputHandler;
use crate::item::default::*;
use crate::sort::sort_ty;
use crate::stat::total::Totals;
use crate::tree::Tree;

pub struct WalkDirs<'a> {
    tree: &'a mut Tree,
    path: &'a Path,
    total: &'a mut Totals,
    handle: &'a mut OutputHandler,
    flags: &'a Flags,
}

impl<'a> WalkDirs<'a> {
    pub(crate) fn new(
        tree: &'a mut Tree,
        path: &'a Path,
        total: &'a mut Totals,
        handle: &'a mut OutputHandler,
        flags: &'a Flags,
    ) -> WalkDirs<'a> {
        WalkDirs {
            tree,
            path,
            total,
            handle,
            flags,
        }
    }

    pub(crate) fn walk_dirs(&mut self) {
        let mut entries: Vec<_> = fs::read_dir(self.path).expect("Error walking").collect();

        // If no, default sort, case-sensitive is used
        sort_ty(&mut entries, &self.flags.sort_ty);

        for (index, entry) in entries.iter().enumerate() {
            // Check for "dot" file
            match entry.as_ref() {
                Ok(entry) => {
                    if Self::check_hidden_file(entry) {
                        self.total.hidden_file += 1;
                        continue;
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error retrieving hidden file (files/dirs's name start with '.') entry: {}",
                        err
                    );
                }
            }

            // Modify current vector for generating tree branch
            if index < entries.len() - 1 {
                self.tree.config.nodes.push(1);
            } else {
                self.tree.config.nodes.push(2);
            };

            // Print branch
            self.tree.print_tree(self.handle, self.flags).unwrap();

            // collect item
            let item =
                ItemCollector::new(entry.as_ref().unwrap(), &self.tree.config.depth.0).unwrap();

            item.get_item(self.flags, self.handle, self.total, self.tree);

            self.tree.config.nodes.pop();
        }
    }

    fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
        let check_hidden = check_hidden.file_name();
        let entry_name = check_hidden.to_string_lossy();

        entry_name.starts_with('.')
    }
}
