mod list;
// mod info;
mod color;
mod filter;
mod hash;
mod info;
mod type_json;

use std::fs::File;

use filter::IgnoreFile;

use crate::list::ListingCalls;
use crate::info::InfoFile;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World");

    Ok(())
}

// struct Flags {
//     // Core flags
//     a_flag: bool, // Flag A
//     d_flag: bool, // Flag D
//     f_flag: bool, // Flag F
//     l_flag: bool, // Flag L
//     p_flag: bool, // Flag P
//     s_flag: bool, // Flag S

//     // Extended flags
//     F_flag: bool, // Flag F
//     u_flag: bool, // Flag U
//     g_flag: bool, // Flag G
//     D_flag: bool, // Flag D
//     q_flag: bool, // Flag Q

//     // Report flags
//     N_flag: bool,  // Flag N
//     Q_flag: bool,  // Flag Q
//     R_flag: bool,  // Flag R
//     h_flag: bool,  // Flag H
//     H_flag: bool,  // Flag H
//     si_flag: bool, // Flag SI

//     // Output flags
//     c_flag: bool,      // Flag C
//     no_indent: bool,   // No indent
//     force_color: bool, // Force color
//     no_color: bool,    // No color

//     // Filesystem flags
//     x_dev: bool,     // Xdev
//     no_report: bool, // No report
//     no_links: bool,  // No links
//     reverse: bool,   // Reverse

//     // Search flags
//     ignore_case: bool, // Ignore case
//     match_dirs: bool,  // Match directories
//     inode_flag: bool,  // Inode flag
//     dev_flag: bool,    // Dev flag

//     // Miscellaneous flags
//     X_flag: bool,     // X flag
//     J_flag: bool,     // J flag
//     ff_links: bool,   // Ff links
//     du_flag: bool,    // Du flag
//     prune_flag: bool, // Prune flag
//     meta_first: bool, // Meta first
//     git_ignore: bool, // Git ignore
// }

static mut FLIMIT: i32 = 0;


// ====================================================================================

// This flag use for command line argument

// Core flags
static mut a_flag: bool = false; // Flag A
static mut d_flag: bool = false; // Flag D
static mut f_flag: bool = false; // Flag F
static mut l_flag: bool = false; // Flag L
static mut p_flag: bool = false; // Flag P
static mut s_flag: bool = false; // Flag S

// Extended flags
static mut F_flag: bool = false; // Flag F
static mut u_flag: bool = false; // Flag U
static mut g_flag: bool = false; // Flag G
static mut D_flag: bool = false; // Flag D
static mut q_flag: bool = false; // Flag Q

// Report flags
static mut N_flag: bool = false;  // Flag N
static mut Q_flag: bool = false;  // Flag Q
static mut R_flag: bool = false;  // Flag R
static mut h_flag: bool = false;  // Flag H
static mut H_flag: bool = false;  // Flag H
static mut si_flag: bool = false; // Flag SI

// Output flags
static mut c_flag: bool = false;      // Flag C
static mut no_indent: bool = false;   // No indent
static mut force_color: bool = false; // Force color
static mut no_color: bool = false;    // No color

// Filesystem flags
static mut x_dev: bool = false;     // Xdev
static mut no_report: bool = false; // No report
static mut no_links: bool = false;  // No links
static mut reverse: bool = false;   // Reverse

// Search flags
static mut ignore_case: bool = false; // Ignore case
static mut match_dirs: bool = false;  // Match directories
static mut inode_flag: bool = false;  // Inode flag
static mut dev_flag: bool = false;    // Dev flag

// Miscellaneous flags
static mut X_flag: bool = false;     // X flag
static mut J_flag: bool = false;     // J flag
static mut ff_links: bool = false;   // Ff links
static mut du_flag: bool = false;    // Du flag
static mut prune_flag: bool = false; // Prune flag
static mut meta_first: bool = false; // Meta first
static mut git_ignore: bool = false; // Git ignore


// ====================================================================================

// FIXME: Global variable?  
//
// struct listingcalls lc;

// FIXME: Mutable?
static host: Option<String> = None; // Initialize with None to represent null or empty
static title: &str = "Directory Tree";
static sp: &str = " ";
static _nl: &str = "\n";
static Hintro: Option<String> = None; // Initialize with None to represent null or empty
static Houtro: Option<String> = None; // Initialize with None to represent null or empty
static file_comment: &str = "#";
static file_pathsep: &str = "/";
static timefmt: Option<String> = None; // Initialize with None to represent null or empty
static charset: Option<&str> = None; // Initialize with None to represent null or empty

// ====================================================================================


static mut pattern: i32 = 0;
static mut maxpattern: i32 = 0;
static mut ipattern: i32 = 0;
static mut maxipattern: i32 = 0;
static mut patterns: Option<Vec<Option<String>>> = None;
static mut ipatterns: Option<Vec<Option<String>>> = None;


// ====================================================================================

// FIXME:

// static args: Vec<String> = env::args().collect();
// static mut slevel = String::new();
// static mut curdir = String::new();
// static mut outfile: Option<File> = None;
// static mut level = 0;
// static mut dirs: Vec<i32> = Vec::new();
// static mut maxdirs = 0;
// static mut ERRORS = 0;

// // Initialize your xpattern array if needed
// static mut xpattern: [u8; PATH_MAX] = [0; PATH_MAX];

// let xpattern = PathBuf::new();


static mut S_LEVEL: Option<String> = None;
static mut CURDIR: Option<String> = None;
static mut OUTFILE: Option<File> = None;
static mut LEVEL: i32 = 0;
// static mut dirs: Vec<i32> = None;
static mut DIRS: Option<Vec<i32>> = None;
static mut MAXDIRS: i32 = 0;
static mut ERRORS: i32 = 0;
static mut XPATTERN: i32 = 4096;
static mut MB_CUR_MAX: i32 = 0;




// ====================================================================================

// FUNCTIONS

// TODO: Separate cmd line's fn in other file

/// Parsing arguments with custom prefixes
/// FIXME:
pub(crate) fn long_arg(
    argv: &[&str],
    i: usize,
    j: &mut usize,
    n: &mut usize,
    prefix: &str,
) -> Option<String> {
    let mut ret: Option<String> = None;
    let len = prefix.len();

    if argv.get(i)?.starts_with(prefix) {
        *j = len;
        let arg = argv[i];
        let arg_len = arg.len();

        if arg.as_bytes().get(*j) == Some(&b'=') {
            *j += 1;
            if let Some(val) = arg.get(*j..) {
                ret = Some(val.to_string());
                *j = arg_len - 1;
            } else {
                eprintln!("tree: Missing argument to {}=", prefix);
                std::process::exit(1);
            }
        } else if let Some(next_arg) = argv.get(*n) {
            ret = Some(next_arg.to_string());
            *n += 1;
            *j = arg_len - 1;
        } else {
            eprintln!("tree: Missing argument to {}", prefix);
            std::process::exit(1);
        }
    }

    ret
}


// ====================================================================================


// FIXME:
type dev_t = u32;

fn unix_getfulltree<'a>(
    d: &'a str,
    lev: u64,
    mut dev: dev_t,
    size: &'a mut i64,
    err: &'a mut Option<String>,
) -> 

// Dummy return to avoid compiler error
Result<(), Box<dyn std::error::Error>>

// Later we will return this
//
// Option<Vec<*mut InfoFile<'a>>> 

{

    *err = None;

    // Initialize variables
    let mut path: Option<String> = None;
    let mut pathsize: i64 = 0;
    let mut ig: Option<IgnoreFile> = None;
    let mut inf: Option<InfoFile> = None;
    let mut dir: Option<Vec<*mut InfoFile>> = None;
    let mut sav: Option<Vec<*mut InfoFile>> = None;
    let mut tmp_pattern: i32 = 0;
    let mut start_rel_path: Option<*const i8> = None;


    if (unsafe { LEVEL } >= 0) && lev > unsafe { LEVEL.try_into().unwrap() } {
        // Later we will return this
        // return None;

        // Dummy
        return Ok(());
    }




    Ok(())
    
}
