#[derive(Debug, PartialEq, Eq)]
pub enum RelocationType {
    Aarch64AdrPrelLo21 = 274,
    // TODO: Add more relocation types
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    pub r#type: RelocationType,
    pub symbol_index: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RelocationAddend {
    pub offset: u64, // Address
    pub info: Info,  // Relocation type and symbol index
    pub addend: i64, // Addend
}
