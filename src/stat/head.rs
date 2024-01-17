use crate::flag::Layout;
use crate::handle::OutputHandler;
use crate::Flags;
use std::io::Write;
use std::path::Path;

pub struct Header<'a> {
    opts: &'a Flags,
    handler: &'a mut OutputHandler,
}

impl<'a> Header<'a> {
    pub(crate) fn new(opts: &'a Flags, handler: &'a mut OutputHandler) -> Header<'a> {
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
            self.get_modified_header(curr_dir);
        } else {
            writeln!(self.handler, "{}/", curr_dir).unwrap_or_default();
        }
    }

    fn get_modified_header(&mut self, curr_dir: String) {
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

        let indented_curr_dir = format!(
            "{:remaining_spaces$}{}{}{}",
            "", self.opts.ansi_co.bright_green, &curr_dir, self.opts.ansi_co.reset_ansi
        );

        // TODO
        write!(self.handler, "\n {}\n    .\n", indented_curr_dir).unwrap_or_default();
    }
}
