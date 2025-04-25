#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum SectionType {
    Null = 0,                   // Section header table entry unused
    ProgBits = 1,               // Program data
    SymTab = 2,                 // Symbol table
    StrTab = 3,                 // String table
    Rela = 4,                   // Relocation entries with addends
    Hash = 5,                   // Symbol hash table
    Dynamic = 6,                // Dynamic linking information
    Note = 7,                   // Notes
    NoBits = 8,                 // Program space with no data (bss)
    Rel = 9,                    // Relocation entries, no addends
    ShLib = 10,                 // Reserved
    DynSym = 11,                // Dynamic linker symbol table
    InitArray = 14,             // Array of constructors
    FiniArray = 15,             // Array of destructors
    PreInitArray = 16,          // Array of pre-constructors
    Group = 17,                 // Section group
    SymTabShndx = 18,           // Extended section indices
    GnuAttributes = 0x6ffffff5, // Object attributes
    GnuHash = 0x6ffffff6,       // GNU-style hash table
    GnuVerdef = 0x6ffffffd,     // Version definition section
    GnuVerneed = 0x6ffffffe,    // Version needs section
    GnuVersym = 0x6fffffff,     // Version symbol table
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionFlag {
    Undefined = 0,        // Undefined
    Write = 1 << 0,       // Writable
    Alloc = 1 << 1,       // Occupies memory during execution
    ExecInstr = 1 << 2,   // Executable
    Merge = 1 << 4,       // Might be merged
    Strings = 1 << 5,     // Contains nul-terminated strings
    InfoLink = 1 << 6,    // `sh_info' contains SHT index
    LinkOrder = 1 << 7,   // Preserve order after combining
    Group = 1 << 9,       // Section is member of a group
    Tls = 1 << 10,        // Section hold thread-local data
    Compressed = 1 << 11, // Section with compressed data
}

/// Represents the header of an ELF section.
///
/// This structure contains metadata about a section in an ELF file, including its name,
/// type, flags, memory address, file offset, size, and other attributes.
///
/// # Example: Relationship Between Section Header and .text Section
///
/// ```text
/// ELF File Layout:
/// +-------------------+
/// | ELF Header        |
/// +-------------------+
/// | Section Header 1  |--> Describes .text section
/// +-------------------+
/// | Section Header 2  |
/// +-------------------+
/// | Section Header 3  |
/// +-------------------+
/// | ...               |
/// +-------------------+
/// | .text Section     |--> Contains executable code
/// +-------------------+
/// | ...               |
/// +-------------------+
///
/// Section Header 1 (for .text):
/// +-------------------+
/// | Name Index: 1     |--> Points to ".text" in the string table
/// +-------------------+
/// | Type: PROGBITS    |--> Indicates this section contains program code
/// +-------------------+
/// | Flags: EXEC       |--> Executable section
/// +-------------------+
/// | Address: 0x1000   |--> Virtual memory address during execution
/// +-------------------+
/// | Offset: 0x200     |--> Offset in the ELF file where .text starts
/// +-------------------+
/// | Size: 0x300       |--> Size of the .text section in bytes
/// +-------------------+
///
/// .text Section:
/// +-------------------+
/// | Machine Code      |--> Raw binary instructions for execution
/// +-------------------+
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    /// Index of the section name in the string table.
    pub name_idx: u32,
    /// Name of the section.
    pub name: String,
    /// Type of the section.
    pub r#type: SectionType,
    /// Flags associated with the section.
    pub flags: Vec<SectionFlag>,
    /// Virtual memory address of the section during execution.
    pub addr: u64,
    /// Offset of the section in the ELF file.
    pub offset: u64,
    /// Size of the section in bytes.
    pub size: u64,
    /// Index of a related section, if applicable.
    pub link: u32,
    /// Additional information about the section.
    pub info: u32,
    /// Alignment of the section in memory.
    pub addralign: u64,
    /// Size of each entry in the section, if it holds a table.
    pub entsize: u64,
    /// Raw data contained in the section.
    pub section_raw_data: Vec<u8>,
}
