use std::fmt::Display;
use std::io::Write;
use std::time::Instant;

use crate::handle::OutputHandler;

#[derive(Debug, Default)]
pub struct Totals {
    pub directories: usize,
    pub files: usize,
    pub size: u64,
    pub hidden_file: usize,
}

impl Totals {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn stats(
        self,
        handler: &mut OutputHandler,
        start_time: Instant,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let seconds = (start_time.elapsed()).as_secs() as f64
            + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

        let gigabytes = self.size as f64 / 1_073_741_824.0;

        writeln!(handler)?;
        writeln!(handler, "\nStatistics:")?;
        writeln!(handler, " - Processing Time   : {:?} seconds", seconds)?;
        writeln!(
            handler,
            " - Total Directories : {}",
            format_with_commas(self.directories)
        )?;
        writeln!(
            handler,
            " - Total Files       : {}",
            format_with_commas(self.files)
        )?;
        writeln!(
            handler,
            " - Hidden Files      : {}",
            format_with_commas(self.hidden_file)
        )?;
        writeln!(
            handler,
            " - Total Items       : {}",
            format_with_commas(self.files + self.directories)
        )?;
        writeln!(
            handler,
            " - Total Size        : {:.2} GB ({} bytes)",
            gigabytes,
            format_with_commas(self.size)
        )?;
        writeln!(handler)?;

        Ok(())
    }
}

fn format_with_commas<T: Display>(num: T) -> String {
    let mut formatted = String::new();
    let num_str = num.to_string();

    for (i, c) in num_str.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted.insert(0, ',');
        }
        formatted.insert(0, c);
    }

    formatted
}
