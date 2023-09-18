use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

pub struct FileInfo {
    pub name: Option<String>,
    link: Option<String>,
    is_directory: bool,
    is_sok: bool,
    is_fifo: bool,
    is_executable: bool,
    is_orphan: bool,
    mode: FileMode,
    link_mode: FileMode,
    user_id: UserId,
    group_id: GroupId,
    pub size: FileSize,
    access_time: Timestamp,
    pub creation_time: Timestamp,
    pub modification_time: Timestamp,
    device_id: DeviceId,
    link_device_id: DeviceId,
    inode: Inode,
    link_inode: Inode,
    #[cfg(target_os = "os2")]
    attribute: c_long,
    error_message: Option<String>,
    tag: Option<String>,
    comments: Option<Vec<String>>,
    children: Option<Vec<Box<FileInfo>>>,
    next: Option<Box<FileInfo>>,
    temporary_children: Option<Box<FileInfo>>,
}

type FileMode = u32;
type UserId = u32;
type GroupId = u32;
type FileSize = i64;
type Timestamp = i64;
type DeviceId = u32;
type Inode = u64;

// impl FileInfo {
//     fn new() -> Self {
//         FileInfo {
//             name: None,
//             link: None,
//             is_directory: false,
//             is_sok: false,
//             is_fifo: false,
//             is_executable: false,
//             is_orphan: false,
//             mode: 0,
//             link_mode: 0,
//             user_id: 0,
//             group_id: 0,
//             size: 0,
//             access_time: 0,
//             creation_time: 0,
//             modification_time: 0,
//             device_id: 0,
//             link_device_id: 0,
//             inode: 0,
//             link_inode: 0,
//             #[cfg(target_os = "os2")]
//             attribute: 0,
//             error_message: None,
//             tag: None,
//             comments: None,
//             children: None,
//             next: None,
//             temporary_children: None,
//         }
//     }
// }

// fn main() {
//     // Create an instance of the FileInfo struct
//     let mut file_info = FileInfo::new();

//     // Example usage:
//     file_info.name = Some("file.txt".to_string());
//     file_info.is_directory = true;
//     file_info.size = 1024;

//     // Print the name and size
//     match &file_info.name {
//         Some(name) => println!("Name: {}", name),
//         None => println!("Name: N/A"),
//     }

//     println!("Is Directory: {}", file_info.is_directory);
//     println!("Size: {}", file_info.size);
// }
