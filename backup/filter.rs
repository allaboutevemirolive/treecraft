pub(crate) struct Pattern<'a> {
    pattern: &'a str,
    relative: i32,
    next: Option<Box<Pattern<'a>>>,
}

pub(crate) struct IgnoreFile<'a> {
    path: &'a str,
    remove: Option<Box<Pattern<'a>>>,
    reverse: Option<Box<Pattern<'a>>>,
    next: Option<Box<IgnoreFile<'a>>>,
}
