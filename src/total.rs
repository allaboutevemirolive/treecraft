use crate::branch::TreeStructureFormatter;
use crate::handle::OutputHandler;

use std::fmt::Display;
use std::io::Write;
use std::time::Instant;

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
        fmt: TreeStructureFormatter,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Convert nanoseconds to seconds
        let seconds = (start_time.elapsed()).as_secs() as f64
            + (start_time.elapsed()).subsec_nanos() as f64 / 1_000_000_000.0;

        // Convert bytes to gigabytes
        let gigabytes = self.size as f64 / 1_073_741_824.0;

        writeln!(handler)?;
        writeln!(handler, "\n Insights:\n    .")?;
        writeln!(
            handler,
            "    {}Processing Time      : {:?} seconds",
            fmt.branch_mid, seconds
        )?;
        writeln!(
            handler,
            "    {}Visible Dirs         : {}",
            fmt.branch_mid,
            format_with_commas(self.directories)
        )?;
        writeln!(
            handler,
            "    {}Visible Files        : {}",
            fmt.branch_mid,
            format_with_commas(self.files)
        )?;
        writeln!(
            handler,
            "    {}*Hidden Dirs/Files   : {}",
            fmt.branch_mid,
            format_with_commas(self.hidden_file)
        )?;
        writeln!(
            handler,
            "    {}Total Items(excl.*)  : {}",
            fmt.branch_mid,
            format_with_commas(self.files + self.directories)
        )?;
        writeln!(
            handler,
            "    {}Total Size           : {:.2} GB ({} bytes)",
            fmt.branch_end,
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
