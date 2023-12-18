use crate::OutputHandler;
use crate::flag::Flags;
use crate::flag::OptOutput;
use std::io::{self, Write};

#[derive(Clone)]
pub struct Tree {
    /// Points of attachment for leaves and buds
    pub nodes: Vec<i32>,
    /// Represent how far/depth a branch extends
    /// horizontally from the main stem
    pub reach: u32,
    pub branch: Branch,
}

impl Tree {
    pub fn new(nodes: Vec<i32>, reach: u32, branch: Branch) -> Tree {
        Tree {
            nodes,
            reach,
            branch,
        }
    }

    pub fn print_tree(&self, handle: &mut OutputHandler, flags: &Flags) -> io::Result<()> {
        if flags.opt_ty == OptOutput::All {
            write!(handle, "    ")?;
        }

        // INFO: Use usize type for indexing slices, arrays, and vectors.
        for i in 0..=self.reach as usize {
            if let Some(marker) = self.nodes.get(i) {
                match self.nodes.get(i + 1) {
                    Some(_) => {
                        if marker == &1 {
                            // "│   "
                            write!(handle, "{}", self.branch.stem)?;
                        } else {
                            // "    "
                            write!(handle, "{}", self.branch.axil)?;
                        }
                    }
                    None => {
                        if marker == &1 {
                            // "├── "
                            write!(handle, "{}", self.branch.junction)?;
                        } else {
                            // "└── "
                            write!(handle, "{}", self.branch.twig)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    /// End of a branch, `"└── "`
    pub twig: &'static str,
    /// Middle part of a branch, `"├── "`
    pub junction: &'static str,
    /// Empty space between branches, `"    "`
    pub axil: &'static str,
    /// Main structural part of the tree, `"│   "`
    pub stem: &'static str,
}

impl Default for Branch {
    fn default() -> Branch {
        Branch {
            twig: "└── ",
            junction: "├── ",
            axil: "    ",
            stem: "│   ",
        }
    }
}

impl Branch {
    pub fn new() -> Branch {
        Default::default()
    }
}
