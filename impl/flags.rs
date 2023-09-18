
struct Flags {
    // Core flags
    a_flag: bool,     // Flag A
    d_flag: bool,     // Flag D
    f_flag: bool,     // Flag F
    l_flag: bool,     // Flag L
    p_flag: bool,     // Flag P
    s_flag: bool,     // Flag S

    // Extended flags
    F_flag: bool,     // Flag F
    u_flag: bool,     // Flag U
    g_flag: bool,     // Flag G
    D_flag: bool,     // Flag D
    q_flag: bool,     // Flag Q

    // Report flags
    N_flag: bool,     // Flag N
    Q_flag: bool,     // Flag Q
    R_flag: bool,     // Flag R
    h_flag: bool,     // Flag H
    H_flag: bool,     // Flag H
    si_flag: bool,    // Flag SI

    // Output flags
    c_flag: bool,     // Flag C
    no_indent: bool,  // No indent
    force_color: bool,// Force color
    no_color: bool,   // No color

    // Filesystem flags
    x_dev: bool,      // Xdev
    no_report: bool,  // No report
    no_links: bool,   // No links
    reverse: bool,    // Reverse

    // Search flags
    ignore_case: bool,// Ignore case
    match_dirs: bool, // Match directories
    inode_flag: bool, // Inode flag
    dev_flag: bool,   // Dev flag

    // Miscellaneous flags
    X_flag: bool,     // X flag
    J_flag: bool,     // J flag
    ff_links: bool,   // Ff links
    du_flag: bool,    // Du flag
    prune_flag: bool, // Prune flag
    meta_first: bool, // Meta first
    git_ignore: bool, // Git ignore
}


// fn main() {
//     let flags = Flags {
//         // Initialize flags here
//         a_flag: false,
//         d_flag: false,
//         f_flag: false,
//         l_flag: false,
//         p_flag: false,
//         s_flag: false,
//         F_flag: false,
//         u_flag: false,
//         g_flag: false,
//         D_flag: false,
//         q_flag: false,
//         N_flag: false,
//         Q_flag: false,
//         R_flag: false,
//         h_flag: false,
//         H_flag: false,
//         si_flag: false,
//         c_flag: false,
//         no_indent: false,
//         force_color: false,
//         no_color: false,
//         x_dev: false,
//         no_report: false,
//         no_links: false,
//         reverse: false,
//         ignore_case: false,
//         match_dirs: false,
//         inode_flag: false,
//         dev_flag: false,
//         X_flag: false,
//         J_flag: false,
//         ff_links: false,
//         du_flag: false,
//         prune_flag: false,
//         meta_first: false,
//         git_ignore: false,
//     };
    
//     // You can use `flags` as needed in your code
// }