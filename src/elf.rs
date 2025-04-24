pub mod header;
pub mod program_header;
pub mod relocation;
pub mod section;
pub mod segument;
pub mod symbol;

/// Represents an ELF (Executable and Linkable Format) file.
///
/// This structure contains the parsed components of an ELF file, including its header,
/// section headers, symbols, and relocations.
#[derive(Debug)]
pub struct ELF {
    /// The ELF file header, containing metadata about the file.
    pub header: header::Header,
    /// A list of section headers, describing the sections in the ELF file.
    pub section_headers: Vec<section::Header>,
    /// A list of symbols defined in the ELF file.
    pub symbols: Vec<symbol::Symbol>,
    /// A list of relocation entries with addends.
    pub relocations: Vec<relocation::RelocationAddend>,
}
