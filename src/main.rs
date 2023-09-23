mod file_info;
mod list_dirs;
mod print_branch;
pub mod print_filename;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    treecraft::run()?;
    Ok(())
}
