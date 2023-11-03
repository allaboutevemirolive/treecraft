// Collect default metadata

use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::fs::FileType;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::branch::TreeStructureFormatter;
use crate::flag::Flags;
use crate::handle::OutputHandler;
use crate::init::PrintLocation;
use crate::init::WalkDirs;
use crate::total::Totals;

use super::DisplayBrightGreen;
use super::DisplayOsString;

#[derive(Debug)]
pub struct ConfigDefault {
    pub name: OsString,
    pub path: PathBuf,
    pub depth: i32,
    pub file_type: FileType,
    pub size: u64,
}

impl ConfigDefault {
    #[inline(always)]
    pub fn new(entry: &DirEntry, depth: &i32) -> io::Result<Self> {
        let full_path = entry.path();
        let metadata = fs::symlink_metadata(&full_path)?;
        let file_type = entry.file_type()?;

        Ok(ConfigDefault {
            name: full_path
                .file_name()
                .map(|os_str| os_str.to_os_string())
                .unwrap_or_else(|| "Invalid full-path".into()),
            path: full_path.clone(),
            depth: *depth,
            file_type,
            size: metadata.len(),
        })
    }

    pub fn default_visitor(
        &self,
        // info: &Self,
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
