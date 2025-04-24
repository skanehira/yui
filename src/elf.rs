pub mod header;
pub mod program_header;
pub mod relocation;
pub mod section;
pub mod segument;
pub mod symbol;

#[derive(Debug)]
pub struct Elf {
    pub header: header::Header,
    pub section_headers: Vec<section::Header>,
    pub symbols: Vec<symbol::Symbol>,
    pub relocations: Vec<relocation::RelocationAddend>,
}
