use std::fmt;
use std::time::Instant;

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

    pub fn increment_dirs(&mut self) {
        self.dirs += 1;
    }

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
        // write!(
        //     f,
        //     "Total Folders     : {}\n\
        //      Total Files       : {}\n\
        //      Total Items       : {}\n\
        //      Total Size        : {:.2} GB or {} bytes",
        //     self.dirs,
        //     self.files,
        //     self.files + self.dirs,
        //     self.size as f64 / (1024.0 * 1024.0 * 1024.0), // Convert bytes to gigabytes
        //     self.size
        // )?;

        writeln!(f, "Total Folders     : {}", self.dirs)?;
        writeln!(f, "Total Files       : {}", self.files)?;
        writeln!(f, "Total Items       : {}", self.files + self.dirs)?;
        writeln!(
            f,
            "Total Size        : {:.2} GB or {} bytes",
            self.size as f64 / (1024.0 * 1024.0 * 1024.0), // Convert bytes to gigabytes
            self.size
        )
    }
}

/// Print data into `text file`
pub struct DisplayData<'a> {
    start_time: Instant,
    totals: &'a Totals,
}

impl<'a> DisplayData<'a> {
    pub fn new(start_time: Instant, totals: &'a Totals) -> Self {
        DisplayData { start_time, totals }
    }
}

impl<'a> fmt::Display for DisplayData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds = (self.start_time.elapsed()).as_secs() as f64
            + (self.start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

        let gigabytes = self.totals.size as f64 / 1_073_741_824.0;

        writeln!(f)?;
        writeln!(f, "Times Processing  : {:.6}s", seconds)?;
        writeln!(f, "Total Folders     : {}", self.totals.dirs)?;
        writeln!(f, "Total Files       : {}", self.totals.files)?;
        writeln!(
            f,
            "Total Items       : {}",
            self.totals.files + self.totals.dirs
        )?;
        writeln!(
            f,
            "Total Size        : {:.2} GB or {} bytes",
            gigabytes, self.totals.size
        )
    }
}

/// Calculate time to run the task
pub fn calculate_elapsed_seconds(start_time: Instant) -> f64 {
    
    return start_time.elapsed().as_secs() as f64
        + f64::from(start_time.elapsed().subsec_nanos()) / 1_000_000_000.0;
}
