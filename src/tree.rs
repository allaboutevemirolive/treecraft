use crate::flag::Flags;
use crate::flag::OptOutput;
use crate::handle::loc::OutputHandler;
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
    pub config: TreeConfig,
    pub branch: Branch,
}

impl Tree {
    pub fn new(config: TreeConfig, branch: Branch) -> Tree {
        Tree { config, branch }
    }

    pub fn print_tree(&self, handle: &mut OutputHandler, flags: &Flags) -> io::Result<()> {
        // TODO: We can implement more implementations like, checking
        // file's permission etc.
        if flags.opt_ty == OptOutput::All {
            write!(handle, "    ")?;
        }

        for i in 0..=self.config.depth.0 {
            if let Some(marker) = self.config.nodes.get(i) {
                match self.config.nodes.get(i + 1) {
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

#[derive(Debug, Copy, Clone)]
pub struct TreeDepth(pub usize);

// TODO
// impl TreeDepth {
//     pub fn root() -> Self {
//         Self(0)
//     }

//     pub fn deeper(self) -> Self {
//         Self(self.0 + 1)
//     }
// }

#[derive(Debug, Clone)]
pub struct TreeConfig {
    /// Points of attachment for leaves and buds
    pub nodes: Vec<i32>,

    /// Represent how far/depth a branch extends
    /// horizontally from the main stem
    pub depth: TreeDepth,
}

impl TreeConfig {
    pub fn new(nodes: Vec<i32>, depth: TreeDepth) -> Self {
        Self { nodes, depth }
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
