use std::{fs, path::Path};
use crate::flag::Options;
use crate::handle::OutputHandler;
use crate::item::default::*;
use crate::sort::sort_ty;
use crate::stat::total::Totals;
use crate::tree::Tree;

pub struct WalkDirs<'a> {
    tree: &'a mut Tree,
    path: &'a Path,
    total: &'a mut Totals,
    handle: &'a mut OutputHandler,
    opts: &'a Options,
}

impl<'a> WalkDirs<'a> {
    #[inline(always)]
    pub(crate) fn new(
        tree: &'a mut Tree,
        path: &'a Path,
        total: &'a mut Totals,
        handle: &'a mut OutputHandler,
        opts: &'a Options,
    ) -> WalkDirs<'a> {
        WalkDirs {
            tree,
            path,
            total,
            handle,
            opts,
        }
    }

    #[inline(always)]
    pub(crate) fn walk_dirs(&mut self) {
        let mut entries: Vec<_> = fs::read_dir(self.path).expect("Error walking").collect();

        // If no, default sort, case-sensitive is used
        sort_ty(&mut entries, &self.opts.sort_ty);

        let len = entries.len();
        entries.iter().zip(0..).for_each(|(entry, index)| {
            match entry.as_ref() {
                // skip hidden file
                Ok(entry) if Self::check_hidden_file(entry) => {
                    self.total.hidden_file += 1;
                }
                Ok(entry) => {
                    // Modify current vector for generating tree branch
                    if index < len - 1 {
                        self.tree.config.nodes.push(1);
                    } else {
                        self.tree.config.nodes.push(2);
                    }

                    // collect item
                    if let Ok(item) = ItemCollector::new(entry, &self.tree.config.depth) {
                        // Print branch after collecting item
                        self.tree.print_tree(self.handle, self.opts).unwrap();

                        item.get_item(self.opts, self.handle, self.total, self.tree);
                    } else {
                        eprintln!("Error creating item");
                    }

                    self.tree.config.nodes.pop();
                }
                Err(err) => {
                    eprintln!(
                        "Error retrieving hidden file (files/dirs' name start with '.') entry: {}",
                        err
                    );
                }
            }
        });
    }

    fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
        let check_hidden = check_hidden.file_name();
        let entry_name = check_hidden.to_string_lossy();

        entry_name.starts_with('.')
    }
}
