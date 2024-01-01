use crate::WalkDirs;
use std::fs::{self, DirEntry, FileType};
use std::io;
use std::io::Write;
use std::path::PathBuf;

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
    pub fn get_item(&self, walker: &mut WalkDirs<'_>) {
        if self.file_type.is_dir() {
            self.process_dir(walker);
        } else {
            self.process_file(walker);
        }

        walker.total.size += self.size;
    }

    #[inline(always)]
    // #[rustfmt::skip]
    // TODO: 'process_dir' and 'process_file' should be a trait
    fn process_dir(&self, walker: &mut WalkDirs<'_>) {
        writeln!(
            walker.handle,
            "{}{}{}",
            walker.opts.ansi_co.bright_green, &self.name, walker.opts.ansi_co.reset_ansi
        )
        .unwrap_or_default();

        walker.total.directories += 1;
        walker.tree.config.depth += 1;

        // Iterate next depth of file, to perform DFS
        WalkDirs::new(
            walker.tree,
            &self.path,
            walker.total,
            walker.handle,
            walker.opts,
        )
        .walk_dirs();
    }

    #[inline(always)]
    fn process_file(&self, walker: &mut WalkDirs<'_>) {
        writeln!(walker.handle, "{}", &self.name).unwrap_or_default();
        walker.total.files += 1;
    }
}
