use crate::flag::OptOutput;
use crate::handle::loc::OutputHandler;
use crate::item::DisplayFormatted;
use crate::item::*;
use crate::loc::PrintLocation;
use crate::Flags;
use std::io::Write;
use std::path::Path;

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
        if self.flags.opt_ty == OptOutput::All {
            self.get_indented_header(curr_dir);
        } else {
            self.get_default_header(curr_dir);
        }
    }

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

        let indented_curr_dir = if self.flags.loc == PrintLocation::File {
            format!(
                "{:width$}{}",
                "",
                DisplayFormatted {
                    content: &curr_dir,
                    format_fn: format_default_ref_string,
                },
                width = remaining_spaces
            )
        } else {
            format!(
                "{:width$}{}",
                "",
                DisplayFormatted {
                    content: &curr_dir,
                    format_fn: format_bright_green_ref_string,
                },
                width = remaining_spaces
            )
        };

        write!(self.handler, "\n {}\n    .\n", indented_curr_dir).unwrap_or_default();
    }

    fn get_default_header(&mut self, curr_dir: String) {
        writeln!(self.handler, "/{}", curr_dir).unwrap_or_default();
    }
}
