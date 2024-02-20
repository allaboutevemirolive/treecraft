use std::fmt;

// Central information of our tree-rs.
pub struct TyCtx {}

// TODO: We will wrap GlobalCtxt inside TyCtxt.
pub struct GlobalCtxt {}

// TODO: Combine stat option with indent or not.
pub enum StatCtxt {
    Verbose,
    Middle,
    Simple,
}

pub enum HeadKind<T> {
    Dot,
    DirSlash(T),
    SlashDirSlash(T),
    IndentDir,
}

impl<T> HeadKind<T> {
    #[rustfmt::skip]
    pub fn as_str(&self) -> &'static str {
        match self {
            HeadKind::Dot =>              ".",
            HeadKind::DirSlash(t) =>      "{t}/",
            HeadKind::SlashDirSlash(t) => "",   // &format!("/{}/", t),
            HeadKind::IndentDir =>        "    ",
        }
    }
}

impl<T> fmt::Display for HeadKind<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

pub enum OutTy {
    // Omit ANSI
    File,
    Terminal,
}

pub enum SortKind {
    CaseSensitive,
    CaseInsensitive,
    NoSort,
    NumberSensitive,
    ExtensionSensitive,
}

pub struct TreeCtxt {
    // TyBranch
    // node
}

pub struct BranchCtxt {
    // Branch
    // Depth
}

pub enum Branch {
    Twig,
    Junction,
    Axil,
    Stem,
}

impl Branch {
    #[rustfmt::skip]
    pub fn as_str(&self) -> &'static str {
        match *self {
            Branch::Twig =>     "└── ",
            Branch::Junction => "├── ",
            Branch::Axil =>     "    ",
            Branch::Stem =>     "│   ",
        }
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

// Before print branch
pub enum IndentKind<T> {
    // TODO: Check which situation we need indent
    // - print header
    // - print branch =>
    // - print stat
    // TODO: We can write, indent with verbose stats or indent with simple stats

    // TODO: We can make this as enum. Maybe there is various info we can printout.
    // Maybe use want information for each branch instead of empty space.
    Info(T),
    Indent,
    NoIndent,
    // TODO: This is our default indentation
    // Space(u32),
    Space(usize),
}

impl<T: fmt::Display> IndentKind<T> {
    #[rustfmt::skip]
    pub fn as_str(&self) -> String {
        match &self {
            IndentKind::Info(t) =>  t.to_string(),
            IndentKind::Indent =>   "    ".to_string(),
            IndentKind::NoIndent => "".to_string(),
            IndentKind::Space(v) => " ".repeat(*v),
        }
    }
}

pub enum AnsiKind {
    Reset,
    Green,
    Empty,
}

pub enum DotFolderCtxt {
    Show,
    Hide,
}

// pub enum VectorIter {
//     Branch(LastItem, i32),
// }

// pub enum LastItem {
//     True,
//     False,
// }

// use std::io::BufWriter;
// use std::io::Write;

// impl VectorIter {
//     fn map(&self, out: &mut BufWriter<std::io::Stdout>) {
//         match self {
//             VectorIter::Branch(LastItem::False, 1) => {
//                 write!(out, "│   ").unwrap();
//             }
//             VectorIter::Branch(LastItem::False, 2) => {
//                 write!(out, "    ").unwrap();
//             }
//             VectorIter::Branch(LastItem::True, 1) => {
//                 write!(out, "├── ").unwrap();
//             }
//             VectorIter::Branch(LastItem::True, 2) => {
//                 write!(out, "└── ").unwrap();
//             }
//             _ => {}
//         }
//     }
// }

use std::io::BufWriter;
use std::io::Write;

pub enum VectorCtxt<'vec> {
    LastItem(&'vec i32),
    NotLastItem(&'vec i32),
}

impl<'vec> VectorCtxt<'vec> {
    fn print_branch(&self, out: &mut BufWriter<std::io::Stdout>) {
        match self {
            VectorCtxt::NotLastItem(1) => {
                write!(out, "│   ").unwrap();
            }

            VectorCtxt::NotLastItem(2) => {
                write!(out, "    ").unwrap();
            }

            VectorCtxt::LastItem(1) => {
                write!(out, "├── ").unwrap();
            }

            VectorCtxt::LastItem(2) => {
                write!(out, "└── ").unwrap();
            }
            _ => {}
        }
    }
}

// impl fmt::Display for

// pub enum Integer32 {
//     One,
//     Two,
// }

pub enum EntryKind {
    File,
    Directory,
    Neither,
}

// We do have metadata extractor for linux, unix, windows.
pub struct ItemMetadata {
    pub par_path: std::path::PathBuf,
    pub abs_path: std::path::PathBuf,
    pub rel_path: std::path::PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub permission: String,
}

pub struct Stats {
    pub dirs: usize,
    pub files: usize,
    pub hidden: usize,
    pub acc_size: u64,
}
