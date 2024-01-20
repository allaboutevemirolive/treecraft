use crate::flag::Layout;
use std::io::Write;

#[derive(Clone)]
pub struct Tree {
    pub config: Config,
    pub branch: Branch,
}

impl Tree {
    pub fn new(config: Config, branch: Branch) -> Tree {
        Tree { config, branch }
    }

    /// Print branch based on the vector.
    pub fn print_tree(walk: &mut crate::WalkDir<'_>, index: usize, len: usize) {
        // TODO: We can implement more implementations like, checking
        // file's permission etc.
        if walk.flag.layout_ty == Layout::All {
            write!(walk.std_out, "    ").unwrap();
        }

        // If there is remaining folder needs to traverse
        if index < len - 1 {
            walk.tree.config.nodes.push(1);
        } else {
            walk.tree.config.nodes.push(2);
        }

        // Iterate vector's items
        for (depth, item) in walk.tree.config.nodes.iter().enumerate() {
            // Check if there is remaining item (in vector) need to traverse
            match walk.tree.config.nodes.get(depth + 1) {
                Some(_) => {
                    if item == &1 {
                        write!(walk.std_out, "{}", walk.tree.branch.stem).unwrap();
                    } else {
                        write!(walk.std_out, "{}", walk.tree.branch.axil).unwrap();
                    }
                }
                None => {
                    if item == &1 {
                        write!(walk.std_out, "{}", walk.tree.branch.junction).unwrap();
                    } else {
                        write!(walk.std_out, "{}", walk.tree.branch.twig).unwrap();
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    /// Points of attachment for leaves and buds
    pub nodes: Vec<i32>,

    /// Represent how far/depth a branch extends
    /// horizontally from the main stem
    pub depth: usize,
}

impl Config {
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
