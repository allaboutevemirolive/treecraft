use self::{head::Header, total::Totals};

pub mod head;
pub mod total;


pub struct Stat<'a> {
    pub header: Header<'a>,
    pub total: Totals,
}

impl<'a> Stat<'a> {
    pub fn new(header: Header<'a>, total: Totals) -> Stat {
        Stat {
            header,
            total,
        }
    }
}
