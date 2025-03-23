pub mod header;
pub mod section;
pub mod symbol;

pub struct Elf {
    pub header: header::Header,
    pub section_headers: Vec<section::Header>,
    pub symbols: Vec<symbol::Symbol>,
}
