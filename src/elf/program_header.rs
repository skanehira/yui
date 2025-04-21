use crate::elf::segument;

#[derive(Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    pub r#type: segument::Type,     // Segment type
    pub flags: Vec<segument::Flag>, // Segment flags
    pub offset: u64,                // Segment file offset
    pub vaddr: u64,                 // Segment virtual address
    pub paddr: u64,                 // Segment physical address
    pub filesz: u64,                // Segment size in file
    pub memsz: u64,                 // Segment size in memory
    pub align: u64,                 // Segment alignment
}
