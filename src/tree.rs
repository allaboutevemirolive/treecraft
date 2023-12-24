use crate::flag::Flags;
use crate::flag::OptOutput;
use crate::handle::handle::OutputHandler;
use std::io::{self, Write};

/// Tree's components
trait TreeCmp {
    fn stem(&self) -> &str;

    fn axil(&self) -> &str;

    fn junction(&self) -> &str;

    fn twig(&self) -> &str;
}

#[derive(Clone)]
pub struct Tree {
    // TODO: Better documentation
    /// Points of attachment for leaves and buds
    pub nodes: Vec<i32>,
    /// Represent how far/depth a branch extends
    /// horizontally from the main stem
    pub reach: u32,
    // TODO
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
        // TODO: We can implement more implementations like, checking
        // file's permission etc.
        if flags.opt_ty == OptOutput::All {
            write!(handle, "    ")?;
        }

        // INFO: Use usize type for indexing slices, arrays, and vectors.
        for i in 0..=self.reach as usize {
            if let Some(marker) = self.nodes.get(i) {
                match self.nodes.get(i + 1) {
                    Some(_) => {
                        if marker == &1 {
                            write!(handle, "{}", self.stem())?;
                        } else {
                            write!(handle, "{}", self.axil())?;
                        }
                    }
                    None => {
                        if marker == &1 {
                            write!(handle, "{}", self.junction())?;
                        } else {
                            write!(handle, "{}", self.twig())?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl TreeCmp for Tree {
    /// "│   "
    fn stem(&self) -> &str {
        self.branch.stem
    }

    /// "    "
    fn axil(&self) -> &str {
        self.branch.axil
    }

    /// "├── "
    fn junction(&self) -> &str {
        self.branch.junction
    }

    /// "└── "
    fn twig(&self) -> &str {
        self.branch.twig
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
