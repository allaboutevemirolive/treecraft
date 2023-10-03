#[derive(Debug, Default)]
pub struct Totals {
    pub dirs: usize,
    pub files: usize,
    pub size: u64,
}

impl Totals {
    pub fn new() -> Totals {
        Default::default()
    }
}
