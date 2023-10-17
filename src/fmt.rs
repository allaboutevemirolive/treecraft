use crate::handle::OutputHandler;
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

#[cfg(any(unix, windows))]
#[allow(clippy::cognitive_complexity)]
impl TreeStructureFormatter {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline(always)]
    /// Generate branch with current modified vector
    pub fn print_tree(
        &self,
        node_links: &[i32],
        maxlevel: usize,
        output_handler: &mut OutputHandler,
    ) -> io::Result<()> {
        // Use 'maxlevel' instead of direct 'node_links.len() - 1'
        // to avoid recalculation
        for i in 0..=maxlevel {
            if let Some(marker) = node_links.get(i) {
                if node_links.get(i + 1).is_some() {
                    if marker == &1 {
                        // "│   "
                        write!(output_handler, "{}", self.vertical_bar)?;
                    } else {
                        // "    "
                        write!(output_handler, "{}", self.indent)?;
                    }
                } else {
                    if marker == &1 {
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
