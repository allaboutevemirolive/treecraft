use crate::file::file::OutputType;
use crate::sort::sort::SortType;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Flags {
    // Folder's name
    pub dirname: String,

    // // Listing options
    // aflag: bool,            // -a: All files are listed.
    // dflag: bool,            // -d: List directories only.
    // lflag: bool,            // -l: Follow symbolic links like directories.
    // fflag: bool,            // -f: Print the full path prefix for each file.
    // xflag: bool,            // -x: Stay on the current filesystem only.
    // level: Option<usize>,   // -L level: Descend only level directories deep.
    // Rflag: bool,            // -R: Rerun tree when the max directory level is reached.
    // Pflag: Option<String>,  // -P pattern: List only those files that match the pattern given.
    // Iflag: Option<String>,  // -I pattern: Do not list files that match the given pattern.
    // gitignore: bool,        // --gitignore: Filter by using .gitignore files.
    // gitfile: Option<String>,// --gitfile X: Explicitly read the gitignore file.
    // ignore_case: bool,      // --ignore-case: Ignore case when pattern matching.
    // matchdirs: bool,        // --matchdirs: Include directory names in -P pattern matching.
    // metafirst: bool,        // --metafirst: Print metadata at the beginning of each line.
    // prune: bool,            // --prune: Prune empty directories from the output.
    // info: bool,             // --info: Print information about files found in .info files.
    // infofile: Option<String>,// --infofile X: Explicitly read the info file.
    // noreport: bool,         // --noreport: Turn off file/directory count at the end of the tree listing.
    // charset: Option<String>,// --charset X: Use charset X for terminal/HTML and indentation line output.
    // filelimit: Option<usize>,// --filelimit #: Do not descend dirs with more than # files in them.
    // output: Option<String>, // -o filename: Output to a file instead of stdout.

    // // File options
    // qflag: bool,            // -q: Print non-printable characters as '?'.
    // Nflag: bool,            // -N: Print non-printable characters as is.
    // Qflag: bool,            // -Q: Quote filenames with double quotes.
    // pflag: bool,            // -p: Print the protections for each file.
    // uflag: bool,            // -u: Displays file owner or UID number.
    // gflag: bool,            // -g: Displays file group owner or GID number.
    // sflag: bool,            // -s: Print the size in bytes of each file.
    // hflag: bool,            // -h: Print the size in a more human-readable way.
    // si: bool,               // --si: Like -h, but use SI units (powers of 1000).
    // duflag: bool,           // --du: Compute the size of directories by their contents.
    // Dflag: bool,            // -D: Print the date of the last modification or (-c) status change.
    // timefmt: Option<String>,// --timefmt <f>: Print and format time according to the format <f>.
    // Fflag: bool,            // -F: Appends '/', '=', '*', '@', '|' or '>' as per ls -F.
    // inodes: bool,           // --inodes: Print the inode number of each file.
    // device: bool,           // --device: Print the device ID number to which each file belongs.

    // // Sorting options
    // vflag: bool,            // -v: Sort files alphanumerically by version.
    // tflag: bool,            // -t: Sort files by the last modification time.
    // cflag: bool,            // -c: Sort files by the last status change time.
    // Uflag: bool,            // -U: Leave files unsorted.
    // rflag: bool,            // -r: Reverse the order of the sort.
    // dirsfirst: bool,        // --dirsfirst: List directories before files (-U disables).
    // filesfirst: bool,       // --filesfirst: List files before directories (-U disables).
    // sort: Option<String>,   // --sort X: Select sort: name, version, size, mtime, ctime.
    pub sorttype: SortType,

    // // Graphics options
    // i: bool,                // -i: Don't print indentation lines.
    // A: bool,                // -A: Print ANSI lines graphic indentation lines.
    // S: bool,                // -S: Print with CP437 (console) graphics indentation lines.
    // n: bool,                // -n: Turn colorization off always (-C overrides).
    // C: bool,                // -C: Turn colorization on always.

    // // XML/HTML/JSON options
    // X: bool,                // -X: Prints out an XML representation of the tree.
    // J: bool,                // -J: Prints out a JSON representation of the tree.
    // H: Option<String>,      // -H baseHREF: Prints out HTML format with baseHREF as the top directory.
    // T: Option<String>,      // -T string: Replace the default HTML title and H1 header with string.
    // nolinks: bool,          // --nolinks: Turn off hyperlinks in HTML output.
    // hintro: Option<String>, // --hintro X: Use file X as the HTML intro.
    // houtro: Option<String>, // --houtro X: Use file X as the HTML outro.
    pub output: OutputType,

    // // Input options
    // fromfile: bool,         // --fromfile: Reads paths from files (.=stdin)
    // fromtabfile: bool,      // --fromtabfile: Reads trees from tab indented files (.=stdin).
    // fflinks: bool,          // --fflinks: Process link information when using --fromfile.

    // // Input options
    // fromfile: bool,            // --fromfile: Reads paths from files (.=stdin)
    // fromtabfile: bool,         // --fromtabfile: Reads trees from tab indented files (.=stdin)
    // fflinks: bool,             // --fflinks: Process link information when using --fromfile.

    // // Miscellaneous options
    // version: bool,              // --version: Print version and exit.
    pub help: bool, // --help: Print usage and this help message and exit.
                    // terminator: bool,           // --: Options processing terminator.
}

impl Flags {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn processing_args(&mut self, args: Vec<String>) {
        let mut iter = args.iter().skip(1); // Skip the program name

        // Set default values
        self.dirname = ".".to_string();
        self.output = OutputType::Stdout;

        // Debugging purpose
        // let mut default_sort_type = SortType::default();
        // println!("Default SortType: {:?}", default_sort_type);

        // default_sort_type = SortType::ByFileName;
        // println!("Default SortType: {:?}", default_sort_type);

        for arg in &mut iter {
            if let Some(path) = valid_path(arg) {
                self.dirname = path.to_str().unwrap_or_default().to_string();
            } else {
                match arg.as_str() {
                    // "-a" => self.aflag = true,
                    // "-d" => self.dflag = true,
                    // "-f" => self.fflag = true,
                    // "-l" => self.lflag = true,
                    // "-p" => self.pflag = true,
                    // "-s" => self.sflag = true,
                    // "-F" => self.Fflag = true,
                    // "-u" => self.uflag = true,
                    // "-g" => self.gflag = true,
                    // "-D" => self.Dflag = true,
                    // "-q" => self.qflag = true,
                    // "-N" => self.Nflag = true,
                    // "-Q" => self.Qflag = true,
                    // "-R" => self.Rflag = true,
                    // "-h" => self.hflag = true,
                    // "-H" => self.Hflag = true,
                    // "-si" => self.siflag = true,
                    // "-c" => self.cflag = true,
                    // "--no-indent" => self.noindent = true,
                    // "--force-color" => self.force_color = true,
                    // "--no-color" => self.nocolor = true,
                    // "--xdev" => self.xdev = true,
                    // "--no-report" => self.noreport = true,
                    // "--no-links" => self.nolinks = true,
                    // "--reverse" => self.reverse = true,
                    // "--ignore-case" => self.ignorecase = true,
                    // "--match-dirs" => self.matchdirs = true,
                    // "--inode" => self.inodeflag = true,
                    // "--dev" => self.devflag = true,
                    // "--X" => self.Xflag = true,
                    // "--J" => self.Jflag = true,
                    // "--ff-links" => self.fflinks = true,
                    // "--du" => self.duflag = true,
                    // "--prune" => self.pruneflag = true,
                    // "--meta-first" => self.metafirst = true,
                    // "--git-ignore" => self.gitignore = true,

                    // Output options
                    "-tf" => self.output = OutputType::File,

                    // Sort
                    "-st-fn-lc" => self.sorttype = SortType::default(),
                    "-st-fn" => self.sorttype = SortType::ByFileName,
                    "-st-no" => self.sorttype = SortType::NoSort,

                    // Miscellaneous options
                    "-help" => self.help = true,

                    _ => {
                        break;
                    }
                }
            }
        }
    }
}

fn valid_path(arg: &str) -> Option<PathBuf> {
    let path = Path::new(arg);
    if path.is_dir() || path.is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}
