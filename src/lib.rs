mod list;
// mod info;
mod color;
mod filter;
mod hash;
mod info;
mod type_json;

use filter::IgnoreFile;

use crate::list::ListingCalls;
use crate::info::InfoFile;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World");

    Ok(())
}

struct Flags {
    // Core flags
    a_flag: bool, // Flag A
    d_flag: bool, // Flag D
    f_flag: bool, // Flag F
    l_flag: bool, // Flag L
    p_flag: bool, // Flag P
    s_flag: bool, // Flag S

    // Extended flags
    F_flag: bool, // Flag F
    u_flag: bool, // Flag U
    g_flag: bool, // Flag G
    D_flag: bool, // Flag D
    q_flag: bool, // Flag Q

    // Report flags
    N_flag: bool,  // Flag N
    Q_flag: bool,  // Flag Q
    R_flag: bool,  // Flag R
    h_flag: bool,  // Flag H
    H_flag: bool,  // Flag H
    si_flag: bool, // Flag SI

    // Output flags
    c_flag: bool,      // Flag C
    no_indent: bool,   // No indent
    force_color: bool, // Force color
    no_color: bool,    // No color

    // Filesystem flags
    x_dev: bool,     // Xdev
    no_report: bool, // No report
    no_links: bool,  // No links
    reverse: bool,   // Reverse

    // Search flags
    ignore_case: bool, // Ignore case
    match_dirs: bool,  // Match directories
    inode_flag: bool,  // Inode flag
    dev_flag: bool,    // Dev flag

    // Miscellaneous flags
    X_flag: bool,     // X flag
    J_flag: bool,     // J flag
    ff_links: bool,   // Ff links
    du_flag: bool,    // Du flag
    prune_flag: bool, // Prune flag
    meta_first: bool, // Meta first
    git_ignore: bool, // Git ignore
}

static mut FLIMIT: i32 = 0;

// FIXME: Global variable?
//
// struct listingcalls lc;

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


    Ok(())
    
}
