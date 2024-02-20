pub struct SimpleError {
    pub msg: String,
    pub ty: ErrorKind,
}

pub enum ErrorKind {
    UnknownPath,
    Tree,
    Header,
    Stat,
    Ansi,
}
