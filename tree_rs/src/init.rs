use std::{fs, path::Path, rc::Rc, cell::RefCell};

use crate::{
    flag::Flags,
    handle::OutputHandler,
    item::default::*,
    sort::sort_ty,
    stat::{total::{Totals, self}, Stat},
    tree::{Branch, Tree},
};

pub fn init_() {
    println!("This is init");
}
pub struct WalkDirs<'a> {
    tree: &'a mut Tree,
    path: &'a Path,
    total: &'a mut Totals,
    handler: &'a mut OutputHandler,
    flags: &'a Flags,
}

impl<'a> WalkDirs<'a> {
    pub(crate) fn new(
        tree: &'a mut Tree,
        path: &'a Path,
        total: &'a mut Totals,
        handler: &'a mut OutputHandler,
        flags: &'a Flags,
    ) -> WalkDirs<'a> {
        WalkDirs {
            tree,
            path,
            total,
            handler,
            flags,
        }
    }

    pub(crate) fn walk_dirs(&mut self) {
        let mut entries: Vec<_> = fs::read_dir(self.path).expect("Error walking").collect();

        sort_ty(&mut entries, &self.flags.sort_ty);

        for (index, entry) in entries.iter().enumerate() {
            match entry.as_ref() {
                Ok(entry) => {
                    if WalkDirs::check_hidden_file(entry) {
                        self.total.hidden_file += 1;
                        continue;
                    }
                }
                Err(err) => {
                    eprintln!("Error while retrieving hidden file (files/dirs's name start with '.') entry: {}", err);
                }
            }

            // Modify current vector for generating tree branch
            if index < entries.len() - 1 {
                self.tree.nodes.push(1);
            } else {
                self.tree.nodes.push(2);
            };

            // Print branch
            self.tree.print_tree(self.handler).unwrap();

            // Configure the ways we collect metada
            // let info = ConfigInfo::new(
            //     entry.as_ref().unwrap(),
            //     &self.tree.depth,
            //     &self.flags.config,
            // )?;

            let item = ItemCollector::new(entry.as_ref().unwrap(), &self.tree.reach).unwrap();

            let mut visitor = Visitor::new(&item, &mut self.total, &self.tree, self.handler, self.flags);

            visitor.ty_visitor();

            self.tree.nodes.pop();
        }
    }

    fn check_hidden_file(check_hidden: &fs::DirEntry) -> bool {
        let check_hidden = check_hidden.file_name();
        let entry_name = check_hidden.to_string_lossy();

        entry_name.starts_with('.')
    }
}

pub struct Visitor<'a> {
    pub item: &'a ItemCollector,
    pub total: &'a mut Totals,
    pub tree: &'a Tree,
    pub handler: &'a mut OutputHandler,
    pub flags: &'a Flags,
}

impl<'a> Visitor<'a> {
    pub fn new(
        item: &'a ItemCollector,
        total: &'a mut Totals,
        tree: &'a Tree,
        handler: &'a mut OutputHandler,
        flags: &'a Flags,
    ) -> Visitor<'a> {
        Visitor {
            item,
            total,
            tree,
            handler,
            flags,
        }
    }

    pub fn ty_visitor(&mut self) {
        let tree_copy = self.tree.clone();
        self.item.get_item(self.flags, self.handler, self.total, tree_copy);
    }
}
