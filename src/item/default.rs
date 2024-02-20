use crate::WalkDir;
use std::fs::{self, DirEntry, FileType};
use std::io::Write;
use std::path::PathBuf;

// TODO: Implement specialize crate to collect file metada.

// TODO: Rename to Visitor
pub struct ItemCollector {
    pub name: String,
    // TODO: Is this abosolute path or parent path?
    pub path: PathBuf,
    pub depth: usize,
    // TODO: We need to collect this information and place it in this struct instead.
    // This field is use to check wether our file type is dir or not.
    // Later, we will use this information to process the item.
    pub file_type: FileType,
    pub size: u64,
    // pub is_dir: bool,
    // pub is_file: bool,
}

impl ItemCollector {
    #[inline(always)]
    pub fn new(entry: &DirEntry, depth: &usize) -> ItemCollector {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path).unwrap();
        let file_type = entry.file_type().unwrap();

        ItemCollector {
            name: full_path
                .file_name()
                .expect("Error retrive filename")
                .to_str()
                .expect("Error convert OsStr to &str")
                .to_string(),
            // .map(|os_str| os_str.to_str()),
            // .map(ToString::to_string)
            // .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path,
            depth: *depth,
            file_type,
            size: metadata.len(),
        }
    }

    #[inline(always)]
    pub fn get_item(&self, walk: &mut WalkDir<'_>) {
        if self.file_type.is_dir() {
            self.process_dir(walk);
        } else {
            self.process_file(walk);
        }

        // Update size accumulation
        walk.total.size += self.size;
    }

    // TODO: 'process_dir' and 'process_file' should be a trait?
    #[inline(always)]
    fn process_dir(&self, walk: &mut WalkDir<'_>) {
        write!(
            walk.std_out,
            "{}{}{}",
            walk.flag.ansi_co.bright_green, &self.name, walk.flag.ansi_co.reset_ansi
        )
        .unwrap();

        if walk.flag.show_path {
            write!(walk.std_out, " ──> {}", &self.path.display()).unwrap();
        }

        // Create newline
        writeln!(walk.std_out).unwrap();

        walk.total.directories += 1;

        // Add 1 as we want to traverse the next folder depth
        walk.tree.config.depth += 1;

        // Iterate next depth of file, to perform DFS
        WalkDir::new(walk.tree, &self.path, walk.total, walk.std_out, walk.flag).walk();

        // TODO: We need to use depth from `ItemCollector` instead
        // TODO: Use tail call optimization!
        // Subtract 1 as we fall back from DFS
        // Without this, the depth for current folder is not accurate
        walk.tree.config.depth -= 1;
    }

    #[inline(always)]
    fn process_file(&self, walk: &mut WalkDir<'_>) {
        write!(walk.std_out, "{}", &self.name).unwrap();

        if walk.flag.show_path {
            write!(walk.std_out, " ──> {}", &self.path.display()).unwrap();
        }

        // Create newline
        writeln!(walk.std_out).unwrap();

        walk.total.files += 1;
    }
}
