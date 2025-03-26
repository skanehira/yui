pub mod header;
pub mod relocation;
pub mod section;
pub mod symbol;

pub struct Elf<'a> {
    pub header: header::Header,
    pub section_headers: Vec<section::Header<'a>>,
    pub symbols: Vec<symbol::Symbol>,
    pub relocations: Vec<relocation::RelocationAddend>,
}
