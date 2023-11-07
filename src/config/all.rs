// Collect all metada
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::io;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::branch::TreeStructureFormatter;
use crate::flag::Flags;
use crate::handle::OutputHandler;
use crate::init::PrintLocation;
use crate::init::WalkDirs;
use crate::total::Totals;

use super::DisplayBrightGreen;
use super::DisplayOsString;

#[derive(Debug)]
pub struct ConfigAll {
    pub name: OsString,
    pub path: PathBuf,
    pub depth: i32,
    pub file_type: FileType,
    pub mode: u32,
    pub user_id: u32,
    pub group_id: u32,
    pub size: u64,
    pub device_id: u64,
    pub inode: u64,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub symlink_target: Option<String>,
    pub access_time: i64,
    pub change_time: i64,
    pub modification_time: i64,
}

impl ConfigAll {
    #[inline(always)]
    /// Gather data for each file or folder
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        let (is_symlink, symlink_target) = ConfigAll::get_symlink_info(&full_path, &file_type);

        Ok(ConfigAll {
            name: full_path
                .file_name()
                .map(|os_str| os_str.to_os_string())
                .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            size: metadata.len(),

            mode: metadata.mode(),
            user_id: metadata.uid(),
            group_id: metadata.gid(),

            device_id: metadata.dev(),
            inode: metadata.ino(),
            is_directory: metadata.is_dir(),
            is_symlink,
            symlink_target,
            access_time: metadata.atime(),
            change_time: metadata.ctime(),
            modification_time: metadata.mtime(),
        })
    }

    #[inline(always)]
    // FIXME: Verify if converting this to an OsString is necessary.
    fn get_symlink_info(path: &Path, file_type: &FileType) -> (bool, Option<String>) {
        if file_type.is_symlink() {
            match fs::read_link(path) {
                Ok(target) => (true, Some(target.to_string_lossy().into_owned())),
                Err(_) => (false, None),
            }
        } else {
            (false, None)
        }
    }

    pub fn all_visitor(
        &self,
        flags: &Flags,
        handler: &mut OutputHandler,
        totals: &mut Totals,
        nodes: &mut Vec<i32>,
        fmt: &TreeStructureFormatter,
        depth: &i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.file_type.is_dir() {
            // Avoid ANSI color if printing in a file,
            // but include ANSI when printing to the terminal.
            if flags.output == PrintLocation::File {
                writeln!(handler, "{}", DisplayOsString(&self.name)).unwrap_or_default();
            } else {
                writeln!(handler, "{}", DisplayBrightGreen(&self.name)).unwrap_or_default();
            }

            totals.directories += 1;

            let next_depth = depth + 1;

            let walker = WalkDirs::new(&self.path, nodes, &next_depth, totals, fmt, handler, flags);

            if let Err(err) = walker.walk_dirs() {
                eprintln!("Error: {}", err);
            }
        } else {
            writeln!(handler, "{}", DisplayOsString(&self.name)).unwrap_or_default();
            totals.files += 1;
        }

        totals.size += self.size;

        Ok(())
    }
}
