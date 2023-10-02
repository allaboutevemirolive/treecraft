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






// Define a struct to hold your data
pub struct DisplayData {
    pub seconds: f64,
    pub total_folders: usize,
    pub total_files: usize,
    pub total_size_gb: f64,
    pub total_size_bytes: u64,
}

impl fmt::Display for DisplayData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Times Processing  : {:.6}s\n", self.seconds)?;
        write!(f, "Total Folders     : {}\n", self.total_folders)?;
        write!(f, "Total Files       : {}\n", self.total_files)?;
        write!(f, "Total Items       : {}\n", self.total_files + self.total_folders)?;
        write!(f, "Total Size        : {:.2} GB or {} bytes\n", self.total_size_gb, self.total_size_bytes)
    }
}