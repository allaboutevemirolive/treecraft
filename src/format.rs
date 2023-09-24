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
        dynamic_places: &[i32], 
        maxlevel: usize,
        outfile: &mut String,
    ) {
        for i in 0..=maxlevel {
            if let Some(dir) = dynamic_places.get(i) {
                if dynamic_places.get(i + 1).is_some() {
                    if *dir == 1 {
                        // "│   "
                        outfile.push_str(&self.vertical_bar);
                    } else {
                        // "    "
                        outfile.push_str(&self.indent);
                    }
                } else {
                    if *dir == 1 {
                        // "├── "
                        outfile.push_str(&self.branch_mid);
                    } else {
                        // "└── "
                        outfile.push_str(&self.branch_end);
                    }
                }
            }
        }
    }
    
}
