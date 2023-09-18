// mod sort;
// mod file_info;
// mod flags;
// mod emit;

// FIXME: Maybe we can omit all this call?
mod list;
// mod info;
mod filter;
mod hash;
mod color;
mod info;
mod type_json;

// fn main() {
//     println!("Hello World")
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    treecraft::run()
}


// In our main function, we need to set default option in case
// user didnt provide dir path or any arguments

// Check for the presence of a Gitignore file

// Call emit function