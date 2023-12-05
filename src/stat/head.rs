// use crate::item::DisplayCowStr;
use crate::item::DisplayFormatted;
use crate::item::*;
use crate::loc::PrintLocation;
use crate::Flags;
use crate::OutputHandler;
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
    pub(crate) fn print_header(self) {
        let dir_name = Path::new(&self.flags.dir_path);
        let dir_name_os = dir_name.file_name().unwrap_or_default();

        // TODO: Avoid using "Cow<'_, str>" all over the codebase
        // Needs to convert it to &str early
        // let curr_dir = &dir_name_os.to_string_lossy().deref();
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
                    content: curr_dir,
                    format_fn: format_cow_str,
                },
                width = remaining_spaces
            )
        } else {
            format!(
                "{:width$}{}",
                "",
                DisplayFormatted {
                    content: curr_dir,
                    format_fn: format_bright_green_cow_str,
                },
                width = remaining_spaces
            )
        };

        write!(self.handler, "\n {}\n    .\n", indented_curr_dir).unwrap_or_default();
    }
}
