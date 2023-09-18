struct XTable {
    xid: u32,
    name: String,
    nxt: Option<Box<XTable>>,
}

struct InoTable {
    inode: ino_t,
    device: dev_t,
    nxt: Option<Box<InoTable>>,
}

// Define types for ino_t and dev_t if they are not already defined
type ino_t = u64; // Change to the actual type used on your system
type dev_t = u64; // Change to the actual type used on your system
