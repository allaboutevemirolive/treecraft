use std::fs;
use std::io::{BufWriter, Stdout};
use std::path::Path;

use crate::flag::Flag;
use crate::item::default::ItemCollector;
use crate::sort::ty_sort;
use crate::stat::total::Totals;
use crate::tree::Tree;

pub struct WalkDir<'a> {
    pub tree: &'a mut Tree,
    pub path: &'a Path,
    pub total: &'a mut Totals,
    pub std_out: &'a mut BufWriter<Stdout>,
    pub flag: &'a Flag,
}

impl<'a> WalkDir<'a> {
    #[inline(always)]
    pub(crate) fn new(
        tree: &'a mut Tree,
        path: &'a Path,
        total: &'a mut Totals,
        std_out: &'a mut BufWriter<Stdout>,
        flag: &'a Flag,
    ) -> WalkDir<'a> {
        WalkDir {
            tree,
            path,
            total,
            std_out,
            flag,
        }
    }

    /// Walk the whole directories
    #[inline(always)]
    pub(crate) fn walk(&mut self) {
        // Read current dir contents
        let mut entries: Vec<_> = fs::read_dir(self.path)
            .expect("Cannot read entries")
            .collect();

        // Sort dirs based on user's option
        ty_sort(&mut entries, self);

        // Iterate dirs
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (index, entry.as_ref().unwrap()))
            .for_each(|(index, entry)| {
                // let entry = entry.as_ref().unwrap();
                // TODO: Make skip hidden file as optional
                // By default, we skip hidden_file
                if !self.flag.hidden_file && check_hidden_file(entry) {
                    self.total.hidden_file += 1;
                } else if self.tree.config.depth <= self.flag.depth.limit {
                    // If user didnt pass specific depth limit,
                    // by default, we set depth limit by 5,000,

                    // Printout branches
                    Tree::print_tree(self, index, entries.len());

                    // Collect item
                    ItemCollector::new(entry, &self.tree.config.depth).get_item(self);

                    // TODO:
                    // Pop last item in our vector.
                    // Example:
                    //
                    // vec![1 ,2, 3, 4, 5, 6]
                    //                     ^ will be pop out
                    self.tree.config.nodes.pop();
                }
            });
    }
}

fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
    check_hidden.file_name().to_string_lossy().starts_with('.')
}
