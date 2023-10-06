use crate::handler::OutputHandler;
use std::io::{self, Write};

// Instead of using `String` for string literals,
// we use the `&str` type to save memory,
// since we have many instances of `TreeStructureFormatter`
pub struct TreeStructureFormatter {
    branch_end: &'static str,
    branch_mid: &'static str,
    indent: &'static str,
    vertical_bar: &'static str,
}

impl Default for TreeStructureFormatter {
    fn default() -> Self {
        TreeStructureFormatter {
            branch_end: "└── ",
            branch_mid: "├── ",
            indent: "    ",
            vertical_bar: "│   ",
        }
    }
}

impl TreeStructureFormatter {
    pub fn new() -> Self {
        Default::default()
    }

    /// Generate branch with current modified vector
    pub fn print_tree(
        &self,
        dynamic_places: &[i32],
        maxlevel: usize,
        output_handler: &mut OutputHandler,
    ) -> io::Result<()> {
        for i in 0..=maxlevel {
            if let Some(dir) = dynamic_places.get(i) {
                if dynamic_places.get(i + 1).is_some() {
                    if *dir == 1 {
                        // "│   "
                        write!(output_handler, "{}", self.vertical_bar)?;
                    } else {
                        // "    "
                        write!(output_handler, "{}", self.indent)?;
                    }
                } else {
                    if *dir == 1 {
                        // "├── "
                        write!(output_handler, "{}", self.branch_mid)?;
                    } else {
                        // "└── "
                        write!(output_handler, "{}", self.branch_end)?;
                    }
                }
            }
        }
        Ok(())
    }
}
