use std::io::{self, Write};

use crate::file::file::OutputHandle;

pub struct TreeStructureFormatter {
    // "└── "
    branch_end: String,
    // "├── "
    branch_mid: String,
    // "    "
    indent: String,
    // "│   "
    vertical_bar: String,
}

impl TreeStructureFormatter {
    pub fn new() -> Self {
        Self {
            branch_end: String::from("└── "),
            branch_mid: String::from("├── "),
            indent: String::from("    "),
            vertical_bar: String::from("│   "),
        }
    }

    pub fn print_directory_structure(
        &self,
        dynamic_places: &[i32],
        maxlevel: usize,
        output: &mut OutputHandle, // Change the parameter type
    ) -> io::Result<()> {
        for i in 0..=maxlevel {
            if let Some(dir) = dynamic_places.get(i) {
                if dynamic_places.get(i + 1).is_some() {
                    if *dir == 1 {
                        // "│   "
                        write!(output, "{}", self.vertical_bar)?;
                    } else {
                        // "    "
                        write!(output, "{}", self.indent)?;
                    }
                } else {
                    if *dir == 1 {
                        // "├── "
                        write!(output, "{}", self.branch_mid)?;
                    } else {
                        // "└── "
                        write!(output, "{}", self.branch_end)?;
                    }
                }
            }
        }
        Ok(())
    }
    
    
    
}
