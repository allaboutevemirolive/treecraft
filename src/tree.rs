use crate::flag::Layout;
use crate::flag::Options;
use crate::handle::OutputHandler;
use std::io::{self, Write};

pub static TWIG: &str = "└── ";
pub static JUNCTION: &str = "├── ";
pub static AXIL: &str = "    ";
pub static STEM: &str = "│   ";

/// Tree's components-
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

    pub fn print_tree(&self, handle: &mut OutputHandler, opts: &Options) -> io::Result<()> {
        // TODO: We can implement more implementations like, checking
        // file's permission etc.
        if opts.layout_ty == Layout::All {
            write!(handle, "    ")?;
        }

        for i in 0..=self.config.depth {
            if let Some(marker) = self.config.nodes.get(i) {
                match self.config.nodes.get(i + 1) {
                    Some(_) => {
                        if marker == &1 {
                            write!(handle, "{}", STEM)?;
                        } else {
                            write!(handle, "{}", AXIL)?;
                        }
                    }
                    None => {
                        if marker == &1 {
                            write!(handle, "{}", JUNCTION)?;
                        } else {
                            write!(handle, "{}", TWIG)?;
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
pub struct TreeConfig {
    /// Points of attachment for leaves and buds
    pub nodes: Vec<i32>,

    /// Represent how far/depth a branch extends
    /// horizontally from the main stem
    pub depth: usize,
}

impl TreeConfig {
    pub fn new(nodes: Vec<i32>, depth: usize) -> Self {
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
    #[rustfmt::skip]
    fn default() -> Branch {
        Branch {
            twig:     "└── ",
            junction: "├── ",
            axil:     "    ",
            stem:     "│   ",
        }
    }
}

impl Branch {
    pub fn new() -> Branch {
        Default::default()
    }
}

// pub enum BranchPart {
//     Twig,
//     Junction,
//     Axil,
//     Stem,
// }

// impl BranchPart {
//     pub fn get_part(&self) -> &str {
//         match self {
//             BranchPart::Twig => TWIG,
//             BranchPart::Junction => JUNCTION,
//             BranchPart::Axil => AXIL,
//             BranchPart::Stem => STEM,
//         }
//     }
// }
