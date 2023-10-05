#[derive(Debug, Default)]
pub struct Totals {
    pub directories: usize,
    pub files: usize,
    pub size: u64,
}

impl Totals {
    pub fn new() -> Totals {
        Default::default()
    }
}
