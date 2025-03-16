pub mod header;
pub mod section;

pub struct Elf {
    pub header: header::Header,
}
