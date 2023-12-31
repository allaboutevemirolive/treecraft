use crate::ansi::*;
use crate::flag::Layout;
use crate::handle::OutputHandler;
use crate::Location;
use crate::Options;
use std::io::Write;
use std::path::Path;

pub struct Header<'a> {
    opts: &'a Options,
    handler: &'a mut OutputHandler,
}

impl<'a> Header<'a> {
    pub(crate) fn new(opts: &'a Options, handler: &'a mut OutputHandler) -> Header<'a> {
        Header { opts, handler }
    }

    /// Print the name and full path of the target directory
    /// or the current dir if none is specified.
    pub(crate) fn print_header(mut self) {
        let dir_name = Path::new(&self.opts.target_dir);

        // Get current dir
        let curr_dir = dir_name
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map_or_else(String::default, String::from);

        // Example:
        //
        // release
        //    .
        //    ├── build
        //    ├── deps
        //

        // TODO: Seperate header between 'All' and 'Default'
        if self.opts.layout_ty == Layout::All {
            self.get_indented_header(curr_dir);
        } else {
            self.get_default_header(curr_dir);
        }
    }

    #[rustfmt::skip]
    fn get_indented_header(&mut self, curr_dir: String) {
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

        // Padding 'remaining spaces' in front of 'curr_dir' to get
        // better alligntment.
        let remaining_spaces = if curr_dir_len < total_spaces {
            total_spaces - curr_dir_len
        } else {
            0
        };

        let indented_curr_dir = if self.opts.loc == Location::File {
            format!("{:remaining_spaces$}{}", "", &curr_dir)
        } else {
            format!("{:remaining_spaces$}{}{}{}", "", BRIGHT_GREEN, &curr_dir, ANSI_RESET)
        };

        // TODO
        write!(self.handler, "\n {}\n    .\n", indented_curr_dir).unwrap_or_default();
    }

    // TODO
    fn get_default_header(&mut self, curr_dir: String) {
        writeln!(self.handler, "/{}", curr_dir).unwrap_or_default();
    }
}
