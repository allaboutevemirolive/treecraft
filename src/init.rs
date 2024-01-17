use crate::flag::Flags;
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
    pub opts: &'a Flags,
}

impl<'a> WalkDirs<'a> {
    #[inline(always)]
    pub(crate) fn new(
        tree: &'a mut Tree,
        path: &'a Path,
        total: &'a mut Totals,
        handle: &'a mut OutputHandler,
        opts: &'a Flags,
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

        // TODO: Implement small optimization
        // if len is 1, we can skip all check and directly print tree.
        // This is benaficial if the dir we want to traverse is something
        // like java projects, where java has a lot of 'a folder' branch
        //
        // if len == 1 {
        //     self.end_list();
        //     self.print_branch();
        //     let ff = &entries[0];

        //     // ItemCollector::new(&ff.clone().unwrap(), &self.file_depth())
        //     //                 .unwrap()
        //     //                 .get_item(self);
        // }

        sort_ty(&mut entries, self);

        entries.iter().zip(0..).for_each(|(entry, index)| {
            match entry.as_ref() {
                Ok(entry) => {
                    // TODO
                    if check_hidden_file(entry) {
                        self.total.hidden_file += 1;
                    } else {
                        self.tree
                            .print_tree(self.handle, self.opts, index, entries.len())
                            .unwrap();

                        // collect item
                        ItemCollector::new(entry, &self.file_depth())
                            .unwrap()
                            .get_item(self);

                        self.pop_node();
                    }
                }
                Err(err) => {
                    writeln!(self.handle, "{}", err).unwrap_or_default();
                }
            }
        });
    }

    fn pop_node(&mut self) {
        self.tree.config.nodes.pop();
    }
    /// Skip hidden file
    fn file_depth(&self) -> usize {
        self.tree.config.depth
    }
}

fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
    check_hidden.file_name().to_string_lossy().starts_with('.')
}
