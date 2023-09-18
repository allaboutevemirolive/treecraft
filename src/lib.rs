mod util;
use util::flag::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello World");

    // Create a FlagValues instance with default values
    let mut flag_values = FlagValues::new();

    // Set some flags to true
    flag_values.set_flag_true(Flags::dflag);
    flag_values.set_flag_true(Flags::lflag);

    flag_values.set_flag_false(Flags::dflag);

    // Check if a flag is set
    if flag_values.is_flag_set(Flags::dflag) {
        println!("DFlag is set to true");
    }

    if flag_values.is_flag_set(Flags::lflag) {
        println!("LFlag is set to true");
    }

    Ok(())
}

// REFERENCES:
// https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
