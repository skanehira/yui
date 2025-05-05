#[derive(Debug, PartialEq, Eq)]
pub enum RelocationType {
    Aarch64AdrPrelLo21 = 274,
    Aarch64AdrPrelPgHi21 = 275,
    Aarch64AdrAbsLo12Nc = 277,
    Aarch64Call26 = 283,
    Aarch64AdrGotPage = 311,
    Aarch64AdrGotLo12Nc = 312,
    // TODO: Add more relocation types
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    pub r#type: RelocationType,
    pub symbol_index: u32,
}

/// Represents an ELF relocation entry with an explicit addend (Rela).
///
/// This struct corresponds to the Elf64_Rela structure in the ELF specification.
/// It contains the information needed to relocate a symbol in the object file.
#[derive(Debug, PartialEq, Eq)]
pub struct RelocationAddend {
    /// The location to be modified, expressed as an offset from the beginning
    /// of the section or file where the relocation applies.
    pub offset: u64,
    /// Contains both the symbol index and relocation type.
    /// The specific encoding depends on the target architecture.
    pub info: Info,
    /// The constant addend used to compute the value to be stored into the relocatable field.
    pub addend: i64,
}
