use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
enum PrintMode {
    Normal,
    Quoted,
}

#[derive(Debug)]
struct PrintSettings {
    mode: PrintMode,
    qflag: bool,
    mb_cur_max: usize,
}

impl PrintSettings {
    fn new(mode: PrintMode, qflag: bool, mb_cur_max: usize) -> Self {
        PrintSettings { mode, qflag, mb_cur_max }
    }
}

trait Printable {
    fn print_char(&self, outfile: &mut dyn Write, c: char);
    fn print_string(&self, outfile: &mut dyn Write, s: &str);
}

impl Printable for PrintSettings {
    fn print_char(&self, outfile: &mut dyn Write, c: char) {
        let byte = c as u8;
        match self.mode {
            PrintMode::Normal => {
                if 7 <= byte && byte <= 13 || byte == b'\\' || (byte == b'"' && self.qflag) || (byte == b' ' && !self.qflag) {
                    write!(outfile, "\\{}", match byte {
                        7..=13 => ['a', 'b', 't', 'n', 'v', 'f', 'r'][(byte - 7) as usize],
                        _ => byte as char,
                    }).ok();
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

fn is_printable(c: char) -> bool {
    c.is_ascii_graphic() || c.is_ascii_whitespace()
}

fn main() {
    let outfile = File::create("output.txt").expect("Failed to create the output file");
    let mut outfile = io::BufWriter::new(outfile);

    let settings = PrintSettings::new(PrintMode::Normal, false, 2); // Customize these settings as needed
    let s = "HelloÆ, World!¶"; // Replace with your actual input string

    settings.print_string(&mut outfile, s);
}
