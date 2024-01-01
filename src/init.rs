use crate::flag::Options;
use crate::handle::OutputHandler;
use crate::item::default::*;
use crate::sort::sort_ty;
use crate::stat::total::Totals;
use crate::tree::Tree;
use std::io::Write;
use std::{fs, path::Path};
pub struct WalkDirs<'a> {
    pub tree: &'a mut Tree,
    pub path: &'a Path,
    pub total: &'a mut Totals,
    pub handle: &'a mut OutputHandler,
    pub opts: &'a Options,
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

        sort_ty(&mut entries, self);

        let len = entries.len();

        entries.iter().zip(0..).for_each(|(entry, index)| {
            match entry.as_ref() {
                Ok(entry) => {
                    // TODO
                    if check_hidden_file(entry) {
                        self.total.hidden_file += 1;
                    } else {
                        // Modify current vector for generating tree branch
                        if index < len - 1 {
                            self.not_end_list();
                        } else {
                            self.end_list();
                        }

                        self.print_branch();

                        // collect item
                        ItemCollector::new(entry, &self.file_depth())
                            .unwrap()
                            .get_item(self);

                        self.pop_node();
                    }
                }
                Err(err) => {
                    let _ = writeln!(self.handle, "{}", err);
                }
            }
        });
    }
}

impl<'a> WalkDirs<'a> {
    fn not_end_list(&mut self) {
        self.tree.config.nodes.push(1);
    }

    fn end_list(&mut self) {
        self.tree.config.nodes.push(2);
    }

    fn pop_node(&mut self) {
        self.tree.config.nodes.pop();
    }

    /// Print branch after collecting item
    fn print_branch(&mut self) {
        self.tree.print_tree(self.handle, self.opts).unwrap();
    }

    /// skip hidden file
    fn file_depth(&self) -> usize {
        self.tree.config.depth
    }
}

fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
    check_hidden.file_name().to_string_lossy().starts_with('.')
}
