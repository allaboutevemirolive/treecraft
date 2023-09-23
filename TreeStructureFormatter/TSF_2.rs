use std::io::Write;

/*
"└── "

"├─  "

"    "

"│   "
*/
struct TreeStructureFormatter {
    branch_end: String,
    branch_mid: String,
    indent: String,
    vertical_bar: String,
}

impl TreeStructureFormatter {
    fn new() -> Self {
        Self {
            branch_end: String::from("└── "),
            branch_mid: String::from("├── "),
            indent: String::from("    "),
            vertical_bar: String::from("│   "),
        }
    }

    fn print_directory_structure(
        &self,
        dirs: &[u8],
        maxlevel: usize,
        outfile: &mut std::fs::File,
        Hflag: bool,
    ) {
        for i in 1..=maxlevel {
            if let Some(dir) = dirs.get(i) {
                if dirs.get(i + 1).is_some() {
                    if *dir == 1 {
                        write!(outfile, "{} ", self.vertical_bar).unwrap();
                    } else if Hflag {
                        write!(outfile, "&nbsp;&nbsp;&nbsp; ").unwrap();
                    } else {
                        write!(outfile, "{} ", self.indent).unwrap();
                    }
                } else {
                    if *dir == 1 {
                        write!(outfile, "{} ", self.branch_mid).unwrap();
                    } else {
                        write!(outfile, "{} ", self.branch_end).unwrap();
                    }
                }
            }
        }
    }
}
fn main() {
    let dirs: Vec<Vec<u8>> = vec![vec![1, 1, 1, 1, 1, 1]];
    let treestructureformatter = TreeStructureFormatter::new();
    let outfile = &mut std::fs::File::create("output.txt").unwrap();
    let Hflag = false;

    for dirs_per_level in &dirs {
        let maxlevel = dirs_per_level.len() - 1;
        treestructureformatter.print_directory_structure(dirs_per_level, maxlevel, outfile, Hflag);
    }
}

