use std::fmt::Write as OtherWrite;
pub struct TreeStructureFormatter {
    // "└── "
    branch_end: String,
    // "├── "
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
        dirs: &Vec<u8>, // Change the parameter type to a Vec<i32> reference
        maxlevel: usize,
        outfile: &mut String,
        Hflag: bool,
    ) {
        for i in 0..=maxlevel {
            if let Some(dir) = dirs.get(i) {
                if dirs.get(i + 1).is_some() {
                    if *dir == 1 {
                        // "│   "
                        outfile.push_str(&self.vertical_bar);
                        outfile.push(' ');
                    } else if Hflag {
                        outfile.push_str("&nbsp;&nbsp;&nbsp; ");
                    } else {
                        // "    "
                        outfile.push_str(&self.indent);
                        outfile.push(' ');
                    }
                } else {
                    if *dir == 1 {
                        // "    "
                        outfile.push_str(&self.branch_mid);
                        outfile.push(' ');
                    } else {
                        // "├── "
                        outfile.push_str(&self.branch_end);
                        outfile.push(' ');
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
