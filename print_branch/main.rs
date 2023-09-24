pub mod print_branch;
pub mod list_dirs;
pub mod dyn_listdir;
use crate::{print_branch::*, list_dirs::*, dyn_listdir::*};


/*

2 = └──

1 = ├──

0 = │


0 and 2 has identical property, but for the sake of brevity, 
we separate it 

*/

fn main() {
    let dirs: Vec<Vec<u8>> = vec![

        /*
        
        In real programme, we will use only a vector instead of using 
        multiple vector

        Then, we dynamically modify the same vector

         */
        
        // vec![0],
        // vec![0, 1],
        // vec![0, 1],
        // vec![0, 1],
        // vec![0, 1, 1],
        // vec![0, 1, 1],
        // vec![0, 1, 2],
        // vec![0, 1],
        // vec![0, 1],
        // vec![0, 1, 1],
        // vec![0, 1, 2],
        // vec![0, 1, 2, 2],
        // vec![0, 1],
        // vec![0, 2],
        // vec![0, 2, 1],
        // vec![0, 2, 1],
        // vec![0, 2, 2],
        // vec![0, 2, 2, 1],
        // vec![0, 2, 2, 2],

        // // /1_first
        // vec![1],
        // // /1_first/cat.rs
        // vec![1, 2],
        // // /2_second
        // vec![0],
        // // /2_second/ecak.rs
        // vec![0, 2],


        // vec![1],
        // vec![1, 2],
        // // vec![0, 1, 1],



        vec![1],
vec![1, 1],
vec![1, 1],
vec![1, 1],
vec![1, 1],
vec![1, 2],
vec![1],
vec![1, 1],
vec![1, 1],
vec![1, 1],
vec![1, 2],
vec![1],
vec![1, 2],
vec![1, 2, 1],
vec![1, 2, 2],
vec![1, 2, 2, 2],
vec![1, 2, 2, 2, 1],
vec![1, 2, 2, 2, 2],
vec![1],
vec![1],
vec![1],
vec![1, 1],
vec![1, 2],
vec![1],
vec![1, 1],
vec![1, 1],
vec![1, 1],
vec![1, 2],
vec![1],
vec![1, 1],
vec![1, 1],
vec![1, 1],
vec![1, 2],
vec![2],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 1],
vec![2, 2],








        // vec![1],
        // vec![1],
        // vec![1, 2],

        // // vec![0, 2],

        // vec![1],
        // vec![1, 2],
        // vec![1, 2, 1],
        // vec![1, 2, 2],

        // vec![1],
        // vec![1],
        // vec![1, 1],
        // vec![1, 2 ],
        // vec![1, 2, 2 ],


        // vec![2],
        






        /*


        2 = └──

        1 = ├──

        0 = │
        
        
        let directory_path = "/home/nemesis/Documents/Github/my_repo/DynamicTreeView/test_folder";
        
        let mut dynamic_places: Vec<i32> = vec![0; 500];

        let sub_directory = read_dir(directory_path)?;

        recursive(sub_directory)

        fn recursive(dir) {

            let sub_directory = read_dir(dir)?;

            for (index, dir) in sub_directory.iter().enumerate() {

                print_branch()

                has_subdir = 0;

                if sub_directory.get(index + 1).is_some() {
                    dynamic_places[max_level] = 1 ;
                } else {
                    dynamic_places[max_level] = 2 ;
                }



                if dir.is_folder {

                    total.folders++

                    has_subdir = 1;


                    new_path = dir / dir.path.clone();



                    // Do some check if dir is empty or not
                    // If inside dir is empty, has_subdir = 0


                    if has_subdir > 0 {

                        let sub_directory = read_dir(dir)?;

                    }

                    if (subdir == NULL) {
                        has_subdir = 0;
                    }



                    

                } else {
                    total.files++

                }



                print_filename



                if has_subdir > 0 {

                    print_newline


                    recursive(new_path);
                }

                if sub_directory.get(index + 1).is_some() && !sub_directory.get(index + 2).is_some() {}

                    dynamic_places[max_level] = 2;
                }
        
        
            }

        }
        
        
        
        
        
        
         */


        
        
    ];


    let directory_path = "/home/nemesis/Documents/Github/my_repo/DynamicTreeView/test_folder";

    let level = 1;

    let sub_directory = read_directory_recursive(directory_path, level).unwrap();

    // Initialize vector for dynamic manipulation
    let mut dynamic_places: Vec<i32> = vec![0; 500];

    let mut outfile = String::from("");

    let Hflag = false;

    // for (index, dir) in sub_directory.iter().enumerate() {
    //     let max_level = dir.level as usize;

    //     if sub_directory.get(index + 1).is_some() {
    //         dynamic_places[max_level] = 1 ;
    //     } else {
    //         dynamic_places[max_level] = 2 ;
    //     }

    //     // let dynamic_place = [_:0];

    //     for idx in 0..=max_level {
    //         print!("{}", dynamic_places[idx]);
    //     }
    //     print!("\n");

    //     /*
    //     01
    //     011
    //     01
    //     01
    //     012
    //      */


    //     let treestructureformatter = TreeStructureFormatter::new();
        
    //     // let maxlevel = dynamic_places.len() - 1;
    //     treestructureformatter.print_directory_structure(
            
    //         &dynamic_places,
    //         max_level + 1,
    //         &mut outfile,
    //         Hflag,
    //     );
    //     treestructureformatter.new_line(&mut outfile);


    // }

    
    
    










    let treestructureformatter = TreeStructureFormatter::new();
    let mut outfile = String::from("");
    let Hflag = false;

    for dirs_per_level in &dirs {
        let maxlevel = dirs_per_level.len() - 1;
        treestructureformatter.print_directory_structure(
            dirs_per_level,
            maxlevel,
            &mut outfile,
            Hflag,
        );
        treestructureformatter.new_line(&mut outfile);
    }

    println!("{}", outfile); // Print the result
}
