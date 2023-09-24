use std::io::{self, Write, Cursor};

#[derive(Clone, Copy, Debug)]
pub enum PrintMode {
    Normal,
    Quoted,
}

#[derive(Debug)]
pub struct PrintSettings {
    pub mode: PrintMode,
    pub qflag: bool,
    pub mb_cur_max: usize,
}

impl PrintSettings {
    pub fn new(mode: PrintMode, qflag: bool, mb_cur_max: usize) -> Self {
        PrintSettings { mode, qflag, mb_cur_max }
    }
}

pub trait Printable {
    fn print_char(&self, outfile: &mut dyn Write, c: char);
    fn print_string(&self, outfile: &mut dyn Write, s: &str);
}

impl Printable for PrintSettings {
    fn print_char(&self, outfile: &mut dyn Write, c: char) {
        let byte = c as u8;
        match self.mode {
            PrintMode::Normal => {
                if (7 <= byte && byte <= 13)
                    || byte == b'\\'
                    || (byte == b'"' && self.qflag)
                    || (byte == b' ' && !self.qflag)
                {
                    write!(outfile, "\\{}", match byte {
                        7..=13 => ['a', 'b', 't', 'n', 'v', 'f', 'r'][(byte - 7) as usize],
                        _ => byte as char,
                    })
                    .ok();
                } else if is_printable(c) {
                    write!(outfile, "{}", c).ok();
                } else if self.qflag {
                    write!(outfile, "?").ok();
                } else {
                    write!(outfile, "\\{:03o}", byte).ok();
                }
            }
            PrintMode::Quoted => {
                write!(outfile, "\"").ok();
                for c in c.to_string().chars() {
                    if is_printable(c) {
                        write!(outfile, "{}", c).ok();
                    } else if self.qflag {
                        write!(outfile, "?").ok();
                    } else {
                        write!(outfile, "\\{:03o}", c as u8).ok();
                    }
                }
                write!(outfile, "\"").ok();
            }
        }
    }

    fn print_string(&self, outfile: &mut dyn Write, s: &str) {
        for c in s.chars() {
            self.print_char(outfile, c);
        }
    }
}

pub fn is_printable(c: char) -> bool {
    c.is_ascii_graphic() || c.is_ascii_whitespace()
}

// fn main() {
//     // Create a mutable buffer for capturing the output
//     let mut output_buffer = Cursor::new(Vec::new());

//     let settings = PrintSettings::new(PrintMode::Normal, false, 2); // Customize these settings as needed
//     let s = "Hello, World!"; // Replace with your actual input string

//     settings.print_string(&mut output_buffer, s);

//     // Convert the captured output to a String
//     let captured_output = String::from_utf8(output_buffer.into_inner()).expect("Failed to convert captured output to String");

//     // Now you can print the captured output
//     // println!("{}", captured_output);
// }
