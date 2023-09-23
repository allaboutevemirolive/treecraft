use std::fmt::Write as OtherWrite;
pub struct TreeStructureFormatter {
    // "└── "
    branch_end: String,
    // "├─  "
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
        dirs: &[u8],
        maxlevel: usize,
        outfile: &mut String, // Change to take a mutable reference to String
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

    pub fn new_line(&self, outfile: &mut String) { // Change to take a mutable reference to String
        write!(outfile, "\n").unwrap();
    }
}

// fn main() {
//     let dirs: Vec<Vec<u8>> = vec![vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]];
//     let treestructureformatter = TreeStructureFormatter::new();
//     let mut outfile = String::from("");
//     let Hflag = false;

//     for dirs_per_level in &dirs {
//         let maxlevel = dirs_per_level.len() - 1;
//         treestructureformatter.print_directory_structure(dirs_per_level, maxlevel, &mut outfile, Hflag);
//         treestructureformatter.new_line(&mut outfile);
//     }
    
//     println!("{}", outfile); // Print the result
// }
