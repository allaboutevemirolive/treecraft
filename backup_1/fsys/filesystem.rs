

type FileMode = u32;
type UserId = u32;
type GroupId = u32;
type FileSize = i64;
type Timestamp = i64;
type DeviceId = u32;
type Inode = u64;

// 64K paths maximum
static MAXPATH: i32 = 64*1024;

enum FileToken {
    PathSeparator,
    Directory,
    File,
    EndOfPath,
}

pub struct FileInfo {
    pub name: Option<String>,         // Name of the file or directory.
    link: Option<String>,             // Symbolic link target (if applicable).
    is_directory: bool,               // Indicates if it's a directory.
    is_sok: bool,                     // Indicates if it's a symbolic link.
    is_fifo: bool,                    // Indicates if it's a FIFO (named pipe).
    is_executable: bool,              // Indicates if it's an executable file.
    is_orphan: bool,                  // Indicates if it's an orphaned file.
    mode: FileMode,                   // File permissions mode.
    link_mode: FileMode,              // Permissions mode of symbolic link.
    user_id: UserId,                  // User ID of the owner.
    group_id: GroupId,                // Group ID of the owner.
    pub size: FileSize,               // File size in bytes.
    access_time: Timestamp,           // Access time.
    pub creation_time: Timestamp,     // Creation time (if available).
    pub modification_time: Timestamp, // Modification time.
    device_id: DeviceId,              // Device ID of the file system.
    link_device_id: DeviceId,         // Device ID of the link (if applicable).
    inode: Inode,                     // Inode number.
    link_inode: Inode,                // Inode number of the symbolic link (if applicable).
    // FIXME
    // #[cfg(target_os = "os2")]
    attribute: i32,                   // Additional attribute information (OS/2 specific).
    error_message: Option<String>,    // Error message associated with the file or directory.
    tag: Option<String>,              // A tag associated with the file or directory.
    comments: Option<Vec<String>>,    // Comments associated with the file or directory.
    children: Option<Vec<Box<FileInfo>>>,            // Child nodes (if it's a directory).
    next: Option<Box<FileInfo>>,                     // Next node (for traversal).
    temporary_children: Option<Box<FileInfo>>,       // Temporary child nodes (if needed).
}

impl FileInfo {
    // let file_info = FileInfo::new();
    pub fn new() -> Self {
        FileInfo {
            name: None,
            link: None,
            is_directory: false,
            is_sok: false,
            is_fifo: false,
            is_executable: false,
            is_orphan: false,
            mode: 0, // Initialize with default mode value.
            link_mode: 0, // Initialize with default mode value.
            user_id: 0, // Initialize with default user ID.
            group_id: 0, // Initialize with default group ID.
            size: 0, // Initialize with default size.
            access_time: 0, // Initialize with default access time.
            creation_time: 0, // Initialize with default creation time.
            modification_time: 0, // Initialize with default modification time.
            device_id: 0, // Initialize with default device ID.
            link_device_id: 0, // Initialize with default device ID.
            inode: 0, // Initialize with default inode number.
            link_inode: 0, // Initialize with default inode number.
            attribute: 0, // Initialize with default attribute value.
            error_message: None,
            tag: None,
            comments: None,
            children: None,
            next: None,
            temporary_children: None,
        }
    }
}
