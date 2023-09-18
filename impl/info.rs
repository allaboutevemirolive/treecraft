use crate::filter::Pattern;

// struct Pattern {
//     // Define fields for the Pattern struct
// }

struct Comment<'a> {
    pattern: &'a Pattern<'a>,
    desc: Vec<&'a str>, // Use Vec<String> if you want to own the strings
    next: Option<Box<Comment<'a>>>,
}

struct InfoFile<'a> {
    path: &'a str, // Use String if you want to own the string
    comments: Option<Box<Comment<'a>>>,
    next: Option<Box<InfoFile<'a>>>,
}
