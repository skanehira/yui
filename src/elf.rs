pub mod header;
pub mod program_header;
pub mod relocation;
pub mod section;
pub mod segument;
pub mod symbol;

#[derive(Debug)]
pub struct Elf<'a> {
    pub header: header::Header,
    pub section_headers: Vec<section::Header<'a>>,
    pub symbols: Vec<symbol::Symbol>,
    pub relocations: Vec<relocation::RelocationAddend>,
}

#[derive(Debug)]
pub struct LinkedElf<'a> {
    pub header: header::Header,
    pub program_headers: Vec<program_header::ProgramHeader>,
    pub sections: Vec<section::Header<'a>>,
    // pub symbols: Vec<symbol::Symbol>,
    // pub relocations: Vec<relocation::RelocationAddend>,
}
