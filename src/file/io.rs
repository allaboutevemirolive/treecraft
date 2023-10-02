use std::{io::{self, Write, BufWriter}, fs::File};





// pub fn redirect_stdout_to_file(output_file_path: &str) -> io::Result<()> {
//     let output_file = File::create(output_file_path)?;

//     let mut stdout = io::stdout();
//     let stdout_lock = stdout.lock();
//     let mut stdout_buf = io::BufWriter::new(stdout_lock);

//     // io::stdout().flush()?;
//     // stdout_buf.flush()?;
    

//     Ok(stdout_buf)
// }

pub fn redirect_stdout_to_file(output_file_path: &str) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let _stdout_guard = stdout.lock();
    io::stdout().flush()?;
    io::stdout().lock().flush()?;

    // Create a File from the provided path
    let output_file = File::create(output_file_path)?;

    let mut handle = io::BufWriter::new(output_file);

    // Your code continues here...

    Ok(())
}


pub fn drop_handle_and_output_file(handle: BufWriter<&File>, output_file: File) {
    drop(handle);
    drop(output_file);
}