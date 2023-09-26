use std::io::{self, Write};

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
        dynamic_places: &[i32], // Use a slice instead of Vec
        maxlevel: usize,
        outfile: &mut dyn Write,
    ) -> io::Result<()> {
        for i in 0..=maxlevel {
            if let Some(dir) = dynamic_places.get(i) {
                if dynamic_places.get(i + 1).is_some() {
                    if *dir == 1 {
                        // "│   "
                        write!(outfile, "{}", self.vertical_bar)?;
                    } else {
                        // "    "
                        write!(outfile, "{}", self.indent)?;
                    }
                } else {
                    if *dir == 1 {
                        // "├── "
                        write!(outfile, "{}", self.branch_mid)?;
                    } else {
                        // "└── "
                        write!(outfile, "{}", self.branch_end)?;
                    }
                }
            }
        }
        Ok(())
    }
}
