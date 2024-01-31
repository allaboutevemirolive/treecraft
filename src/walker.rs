use crate::flag::Flag;
use crate::item::default::*;
use crate::sort::sort_ty;
use crate::stat::total::Totals;
use crate::tree::Tree;
use std::fs;
use std::io::{BufWriter, Stdout};
use std::path::Path;

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
        let mut entries: Vec<_> = fs::read_dir(self.path).unwrap().collect();

        // Sort dirs based on user's option
        sort_ty(&mut entries, self);

        // TODO: Remove match pattern
        // Iterate dirs
        entries.iter().enumerate().for_each(|(index, entry)| {
            let entry = entry.as_ref().unwrap();
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

                // Pop last item in our vector.
                // Note that we only use 1 vector for the whole operation.
                // Thus, it only pop `last item` of vector
                // not `last list` of folders.
                //
                // Example:
                //
                // vec![1 ,2, 3, 4, 5, 6]
                //                     ^ will be pop out
                //
                self.tree.config.nodes.pop();
            }
        });
    }
}

fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
    check_hidden.file_name().to_string_lossy().starts_with('.')
}
