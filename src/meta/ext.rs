use std::collections::HashMap;


// Later, we will allow use to get extension only
// instead of mixed with tree

pub struct Extensions {
    pub extensions: HashMap<String, usize>,
}

impl Extensions {
    // Function to initialize the HashMap
    pub fn new() -> Self {
        Extensions {
            extensions: HashMap::new(),
        }
    }

    // Function to collect extensions
    pub fn collect_extension(&mut self, extension: Option<String>) {
        *self
            .extensions
            .entry(extension.unwrap_or_default())
            .or_insert(0) += 1;
    }

    // // Function to get the count of a specific extension
    // fn get_extension_count(&self, extension: &str) -> usize {
    //     *self.extensions.get(extension).unwrap_or(&0)
    // }

    // // Function to remove an extension from the collection
    // fn remove_extension(&mut self, extension: &str) -> Option<usize> {
    //     self.extensions.remove(extension)
    // }

    // // Function to clear the entire collection
    // fn clear_extensions(&mut self) {
    //     self.extensions.clear();
    // }

    // // Function to check if a specific extension exists
    // fn contains_extension(&self, extension: &str) -> bool {
    //     self.extensions.contains_key(extension)
    // }

    pub fn sorted_extensions_by_count(&self) -> Vec<(&String, &usize)> {
        let mut extension_vec: Vec<(&String, &usize)> = self.extensions.iter().collect();
        extension_vec.sort_by_cached_key(|&(_, count)| std::cmp::Reverse(count)); // Sort in descending order by count
        extension_vec
    }

    pub fn print_sorted_table(&self) {
        use tabular::{Row, Table};

        // FIXME:
        // Reusing the Table
        // Initialize in struct?
        let mut table: Table = Table::new("{:<} {:>}");

        for (extension, count) in &self.sorted_extensions_by_count() {
            table.add_row(Row::new().with_cell(extension).with_cell(count));
        }

        println!();
        print!("{}", table);
    }
}
