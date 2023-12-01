use crate::Flags;
use crate::OutputHandler;
use std::io::Write;
use std::path::Path;

//#[derive(Clone)]
pub struct Header<'a> {
    flags: &'a Flags,
    handler: &'a mut OutputHandler,
}

impl<'a> Header<'a> {
    #[inline(always)]
    pub(crate) fn new(flags: &'a Flags, handler: &'a mut OutputHandler) -> Header<'a> {
        Header { flags, handler }
    }

    /// Print the name and full path of the target directory
    /// or the current dir if none is specified.
    #[inline(always)]
    pub(crate) fn print_header(mut self) {
        let dir_name = Path::new(&self.flags.dir_path);
        let dir_name_os = dir_name.file_name().unwrap_or_default();
        let curr_dir = &dir_name_os.to_string_lossy();

        //
        // release
        //    .
        //    ├── build
        //    ├── deps
        //

        //
        // Problem if 'go' is not long enough
        //
        // go
        //    .
        //    ├── build
        //    ├── deps
        //

        // What we want:
        //
        //   go
        //    .
        //    ├── build
        //    ├── deps
        //

        // Calculate the number of spaces needed to center the text
        let total_spaces = 4;
        let curr_dir_len = curr_dir.len();
        let remaining_spaces = if curr_dir_len < total_spaces {
            total_spaces - curr_dir_len
        } else {
            0
        };

        // Create a string `indented_curr_dir` where `curr_dir`
        // is right-aligned and padded with spaces on the left to
        // achieve a minimum width specified by `remaining_spaces`.
        // This helps in aligning the text properly.
        // Add remaining spaces in front of curr_dir.
        let indented_curr_dir = format!("{:width$}{}", "", curr_dir, width = remaining_spaces);

        // Print header
        write!(self.handler, "\n {}\n    .\n", indented_curr_dir).unwrap_or_default();
    }
}
