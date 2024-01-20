use crate::Branch;
use std::fmt::Display;
use std::io::{BufWriter, Stdout, Write};
use std::time::Instant;

#[derive(Debug, Default, Clone, Copy)]
pub struct Totals {
    pub directories: usize,
    pub files: usize,
    pub size: u64,
    pub hidden_file: usize,
}

impl Totals {
    pub fn new() -> Totals {
        Default::default()
    }

    pub(crate) fn stats(
        self,
        std_out: &mut BufWriter<Stdout>,
        start_time: Instant,
        branch: Branch,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Convert nanoseconds to seconds
        let seconds = (start_time.elapsed()).as_secs() as f64
            + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

        // Convert bytes to gigabytes
        let gigabytes = self.size as f64 / 1_073_741_824.0;

        writeln!(std_out)?;
        writeln!(std_out, "\n  Insights:\n    .")?;
        writeln!(
            std_out,
            "    {}Processing Time      : {:?} seconds",
            branch.junction, seconds
        )?;
        writeln!(
            std_out,
            "    {}Visible Dirs         : {}",
            branch.junction,
            format_with_commas(self.directories)
        )?;
        writeln!(
            std_out,
            "    {}Visible Files        : {}",
            branch.junction,
            format_with_commas(self.files)
        )?;
        writeln!(
            std_out,
            "    {}*Hidden Dirs/Files   : {}",
            branch.junction,
            format_with_commas(self.hidden_file)
        )?;
        writeln!(
            std_out,
            "    {}Total Items(excl.*)  : {}",
            branch.junction,
            format_with_commas(self.files + self.directories)
        )?;
        writeln!(
            std_out,
            "    {}Total Size           : {:.2} GB ({} bytes)",
            branch.twig,
            gigabytes,
            format_with_commas(self.size)
        )?;
        writeln!(std_out)?;

        Ok(())
    }

    pub fn default_stat(
        self,
        std_out: &mut BufWriter<Stdout>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(std_out)?;
        write!(
            std_out,
            "{} directories",
            format_with_commas(self.directories)
        )?;
        write!(std_out, ", ")?;
        write!(std_out, "{} files", format_with_commas(self.files))?;
        writeln!(std_out)?;
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
