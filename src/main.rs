mod config;
mod fsys;
mod platform;
mod rend;
mod ty;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    treecraft::run()
}
