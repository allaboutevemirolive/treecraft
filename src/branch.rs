use crate::handle::OutputHandler;

use std::io::{self, Write};

// INFO
// Instead of using `String` for string literals,
// we use the `&str` type to save memory,
// since we have many instances of `TreeStructureFormatter`
//
// INFO
// Rust compiler optimize code more aggressively around structs
// compared to using direct initialization of static variables.
// This is because structs provide more information to the compiler
// about the 'layout of data in memory', which allows the compiler to
// make more informed decisions about how to optimize the code.
//
// REF
// https://camlorn.net/posts/April%202017/rust-struct-field-reordering/
// https://lwn.net/Articles/250967/
#[derive(Debug)]
pub struct TreeStructureFormatter {
    /// "└── "
    pub branch_end: &'static str,
    /// "├── "
    pub branch_mid: &'static str,
    /// "    "
    pub indent: &'static str,
    /// "│   "
    pub vertical_bar: &'static str,
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
        nodes: &[i32],
        maxlevel: usize,
        handler: &mut OutputHandler,
    ) -> io::Result<()> {
        // Insert 4 spacebar for alignment purposes
        write!(handler, "    ")?;

        // TODO
        // Explain how this branch works.
        // INFO
        // Use 'maxlevel' instead of direct 'nodes.len() - 1'
        // to avoid recalculation.
        for i in 0..=maxlevel {
            if let Some(marker) = nodes.get(i) {
                if nodes.get(i + 1).is_some() {
                    if marker == &1 {
                        // "│   "
                        write!(handler, "{}", self.vertical_bar)?;
                    } else {
                        // "    "
                        write!(handler, "{}", self.indent)?;
                    }
                } else if marker == &1 {
                    // "├── "
                    write!(handler, "{}", self.branch_mid)?;
                } else {
                    // "└── "
                    write!(handler, "{}", self.branch_end)?;
                }
            }
        }
        Ok(())
    }
}
