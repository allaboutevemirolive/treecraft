use std::fmt;

#[derive(Debug, Default)]
pub struct Totals {
    pub dirs: usize,
    pub files: usize,
    pub size: u64,
}

impl Totals {
    pub fn new() -> Totals {
        Default::default()
    }

    /// Increment dirs count by `1`
    pub fn increment_dirs(&mut self) {
        self.dirs += 1;
    }

    /// Increment file count by `1`
    pub fn increment_files(&mut self) {
        self.files += 1;
    }

    pub fn add_size(&mut self, size: u64) {
        self.size += size;
    }

    pub fn reset(&mut self) {
        *self = Totals::new();
    }
}

impl fmt::Display for Totals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(
            f,
            "Total Folders     : {}\n\
             Total Files       : {}\n\
             Total Items       : {}\n\
             Total Size        : {:.2} GB or {} bytes",
            self.dirs,
            self.files,
            self.files + self.dirs,
            self.size as f64 / (1024.0 * 1024.0 * 1024.0), // Convert bytes to gigabytes
            self.size
        )
    }
}
