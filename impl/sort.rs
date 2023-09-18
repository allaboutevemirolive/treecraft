/// This file is use to port sort function
use crate::file_info::FileInfo;
use std::cmp::Ordering;

struct Sort {
    name: &'static str,
    cmpfunc: fn() -> i32,
}

// fn alnumsort() -> i32 {
//     // Implementation of alnumsort goes here
//     0 // Placeholder return value
// }

// fn versort() -> i32 {
//     // Implementation of versort goes here
//     0 // Placeholder return value
// }

// fn fsizesort() -> i32 {
//     // Implementation of fsizesort goes here
//     0 // Placeholder return value
// }

// fn mtimesort() -> i32 {
//     // Implementation of mtimesort goes here
//     0 // Placeholder return value
// }

// fn ctimesort() -> i32 {
//     // Implementation of ctimesort goes here
//     0 // Placeholder return value
// }

// fn main() {
//     let sorts: [&'static Sort; 6] = [
//         &Sort {
//             name: "name",
//             cmpfunc: alnumsort,
//         },
//         &Sort {
//             name: "version",
//             cmpfunc: versort,
//         },
//         &Sort {
//             name: "size",
//             cmpfunc: fsizesort,
//         },
//         &Sort {
//             name: "mtime",
//             cmpfunc: mtimesort,
//         },
//         &Sort {
//             name: "ctime",
//             cmpfunc: ctimesort,
//         },
//         &Sort {
//             name: "",
//             cmpfunc: || 0, // Placeholder function and empty string for NULL
//         },
//     ];

//     // Rest of your code goes here
// }


// ====================================================================================

// TODO: Create strverscmp function. Refer from C.


// Dummy
static mut REVERSE: Option<bool> = None;

fn alnum_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
    let a_name = &a.name;
    let b_name = &b.name;
    let v = a_name.cmp(b_name);

    if unsafe { REVERSE.unwrap() } {
        v.reverse()
    } else {
        v
    }
}

// fn ver_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
//
// }

// fn modification_time_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
//     let a_modification_time = &a.modification_time;
//     let b_modification_time = &b.modification_time;

//     let mut v;

//     if a_modification_time == b_modification_time {
//         let mut v = a_modification_time.cmp(b_modification_time);
//         return reverse ? -v : v;
//     } else {
        
//     }
// }


// fn modification_time_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
//     if unsafe { (*a).modification_time } == unsafe { (*b).modification_time } {
//         let a_name = unsafe { CString::from_raw((*a).name as *mut c_char) };
//         let b_name = unsafe { CString::from_raw((*b).name as *mut c_char) };
//         let v = a_name.to_string_lossy().cmp(&b_name.to_string_lossy());
//         return if reverse { v.reverse() } else { v };
//     }

//     let v = if unsafe { (*a).mtime } < unsafe { (*b).mtime } {
//         -1
//     } else {
//         1
//     };

//     if reverse {
//         v.cmp(&0).reverse()
//     } else {
//         v.cmp(&0)
//     }
// }

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9ea8a2006269c03792cd21183816e632
fn modification_time_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
    if a.modification_time == b.modification_time {
        let v = a.name.cmp(&b.name);
        return if unsafe { REVERSE.is_some() } { 
            v.reverse() 
        } else { 
            v 
        };
    }

    let v = if a.modification_time < b.modification_time {
        -1
    } else {
        1
    };

    if unsafe { REVERSE.is_some() } {
        v.cmp(&0).reverse()
    } else {
        v.cmp(&0)
    }
}


fn creation_time_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
    if a.creation_time == b.creation_time {
        let v = a.name.cmp(&b.name);
        return if unsafe { REVERSE.is_some() } { 
            v.reverse() 
        } else { 
            v 
        };
    }

    let v = if a.creation_time < b.creation_time {
        -1
    } else {
        1
    };

    if unsafe { REVERSE.is_some() } {
        v.cmp(&0).reverse()
    } else {
        v.cmp(&0)
    }
}

// FIX: Redundant
fn sizecmp(a: i64, b: i64) -> Ordering {
    a.cmp(&b)
}

fn file_size_sort(a: &FileInfo, b: &FileInfo) -> Ordering {
    let size_comparison = sizecmp(a.size, b.size);
    
    if size_comparison == Ordering::Equal {
        let name_comparison = a.name.cmp(&b.name);
        return if unsafe { REVERSE.unwrap() } { name_comparison.reverse() } else { name_comparison };
    }
    
    if unsafe { REVERSE.unwrap() } {
        size_comparison.reverse()
    } else {
        size_comparison
    }
}