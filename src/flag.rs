use std::env::args;

#[derive(Debug, Default)]
struct Flags {
    dirname: String,
    aflag: bool,
    dflag: bool,
    fflag: bool,
    lflag: bool,
    pflag: bool,
    sflag: bool,
    Fflag: bool,
    uflag: bool,
    gflag: bool,
    Dflag: bool,
    qflag: bool,
    Nflag: bool,
    Qflag: bool,
    Rflag: bool,
    hflag: bool,
    Hflag: bool,
    siflag: bool,
    cflag: bool,
    noindent: bool,
    force_color: bool,
    nocolor: bool,
    xdev: bool,
    noreport: bool,
    nolinks: bool,
    reverse: bool,
    ignorecase: bool,
    matchdirs: bool,
    inodeflag: bool,
    devflag: bool,
    Xflag: bool,
    Jflag: bool,
    fflinks: bool,
    duflag: bool,
    pruneflag: bool,
    metafirst: bool,
    gitignore: bool,
}

impl Flags {
    // Constructor to create a new instance of Flags with default values
    fn new() -> Self {
        Default::default()
    }

    fn processing_args(&self, args: Vec<String>) {

        for i in args {
            
        }
    }
}

// fn main() {
//     // Create a mutable instance of the Flags struct with default values
//     let mut flags = Flags::new();

//     // Modify individual fields as needed
//     flags.aflag = true;
//     flags.dflag = true;
//     flags.fflag = true;
//     // ... (modify other fields)

//     // Now you have a Flags struct with modified field values
//     println!("{:?}", flags);
// }
