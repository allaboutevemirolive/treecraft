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
}

fn main() {
    let formatter = TreeStructureFormatter::new();

    println!("Branch End  : {}", formatter.branch_end);
    println!("Branch Mid  : {}", formatter.branch_mid);
    println!("Indent      : {}", formatter.indent);
    println!("Vertical Bar: {}", formatter.vertical_bar);
}
