/// This file is to manage flag from cmd argument

pub enum Flags {
    dflag,
    lflag,
    pflag,
    sflag,
    Fflag,
    aflag,
    fflag,
    uflag,
    gflag,
    qflag,
    Nflag,
    Qflag,
    Dflag,
    inodeflag,
    devflag,
    hflag,
    Rflag,
    //
    Hflag,
    siflag,
    cflag,
    Xflag,
    Jflag,
    duflag,
    pruneflag,
    //
    noindent,
    force_color,
    nocolor,
    xdev,
    noreport,
    nolinks,
    //
    ignorecase,
    matchdirs,
    fromfile,
    metafirst,
    gitignore,
    showinfo,
    //
    reverse,
    fflinks,
}

// Define a struct to hold the flag values
pub struct FlagValues {
    flags: Vec<bool>,
}

// Implement methods for the FlagValues struct
impl FlagValues {
    // Constructor to create a new FlagValues instance with default values
    pub fn new() -> Self {
        let flags = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ];

        FlagValues { flags }
    }

    // Method to set a specific flag to true
    pub fn set_flag_true(&mut self, flag: Flags) {
        self.flags[flag as usize] = true;
    }

    // Method to set a specific flag to false
    pub fn set_flag_false(&mut self, flag: Flags) {
        self.flags[flag as usize] = false;
    }

    // Method to check if a specific flag is true
    pub fn is_flag_set(&self, flag: Flags) -> bool {
        self.flags[flag as usize]
    }
}

// fn main() {
//     // Create a FlagValues instance with default values
//     let mut flag_values = FlagValues::new();

//     // Set some flags to true
//     flag_values.set_flag(Flags::DFlag);
//     flag_values.set_flag(Flags::LFlag);

//     // Check if a flag is set
//     if flag_values.is_flag_set(Flags::DFlag) {
//         println!("DFlag is set to true");
//     }

//     if flag_values.is_flag_set(Flags::LFlag) {
//         println!("LFlag is set to true");
//     }
// }
