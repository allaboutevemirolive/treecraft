pub mod file_info;
pub mod list_dirs;
pub mod print_branch;
pub mod print_filename;
use crate::{
    file_info::*, 
    list_dirs::*, 
    print_branch::*
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo/treecraft/backup_1";

    let mut output_file = String::from("");
    // Get list of sub-directories for the target path
    let sub_directory: Vec<ListDirs> = read_dir(directory_path)?;

    // let sub_directory_count = sub_directory.len();

    // for dir in &sub_directory {

    // }

    // ===========================================================

    // Create a vector to store the metadata for sub-directories
    let mut sub_dirs_metadata: Vec<FileInfo> = Vec::new();
    let level = 1;

    for info in &sub_directory {
        // For each sub-directory, retrieve metadata and store it in sub_dirs_metadata
        let metadata: FileInfo = FileInfo::new(info.path.to_str().unwrap(), level);
        sub_dirs_metadata.push(metadata);
    }

    // for info in &sub_dirs_metadata {
    //     println!("File/Directory Info:");
    //     println!("Name: {}", info.name);
    //     println!("Path: {}", info.path);
    //     println!("Level: {}", info.level);
    //     println!("Mode: {:o}", info.mode);
    //     println!("UID: {}", info.uid);
    //     println!("GID: {}", info.gid);
    //     println!("Size: {} bytes", info.size);
    //     println!("Device ID: {}", info.device_id);
    //     println!("Inode: {}", info.inode);
    //     println!("Is Directory: {}", info.is_directory);
    //     println!("Is Symbolic Link: {}", info.is_symlink);
    //     if let Some(target) = &info.symlink_target {
    //         println!("Symlink Target: {}", target);
    //     }
    //     println!("Access Time: {}", info.access_time);
    //     println!("Change Time: {}", info.change_time);
    //     println!("Modification Time: {}", info.modification_time);

    //     println!("======================================");
    // }

    // for info in &sub_dirs_metadata {

    //     if info.is_directory == true {

    //     }
    // }

    // if let Some(next_ptr) = dirs.get(lev + 1) {
    //     if next_ptr.is_some() {
    //         // The current level is not the last item
    //         dirs[lev] = Some(std::ptr::null_mut()); // Or assign the actual pointer
    //     } else {
    //         // Marks the current level as the last item
    //         dirs[lev] = Some(std::ptr::null_mut()); // Or assign the actual pointer
    //     }
    // }

    let mut sub_directory: Vec<ListDirs> = Vec::new();

    for dir in &sub_dirs_metadata {


        // Print branch
        // - take argument: level

        let treestructureformatter = TreeStructureFormatter::new();

        let Hflag = false;

        // i32 -> usize
        // let mut maxlevel = dir.level.try_into().unwrap();

        // treestructureformatter.print_directory_structure(dir, maxlevel, &mut output_file, Hflag);


        // Initialize default
        let mut has_subdir = 0;

        // Check if dir is folder
        // FIXME: change to "is_folder"
        if dir.is_directory {
            // FIXME: accumulate total dir later

            // we assume in current folder has nested dir
            has_subdir = 1;

            let new_path = dir.path.clone();

            if has_subdir > 0 {
                // we try to retrieve list of dir inside the folder
                sub_directory = read_dir(&new_path)?;

                // FIXME: we didnt handle if there is an error in retrieving list
                if sub_directory.is_empty() {
                    has_subdir = 0;
                }
            }
        } else {
            // is Files, accumulate total files
        }

        

        
    }

    // let my_vector: Vec<Option<i32>> = vec![None; 64000];

    // let dirs: Vec<Vec<Option<i32>>> = vec![None; 64000];

    /*
    "level/depth" is the indicator of how many vector we want to create
    vector index start from 0.

    level is determing by slash  "./home/document"  equal to 3 level

    The value of inside vector by default set to 0. For all inner.
    
     */

    // Should we make it as static?
    // It is safe to initialize all to 0 instead of using some.
    // MaxPath of depth
    let size = 64000;
    let mut dirs: Vec<Vec<Option<i32>>> = vec![vec![Some(0); size]; size];


    Ok(())
}
