#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub name_idx: u32,           // Section name (string tbl index)
    pub name: String,            // Section name
    pub r#type: SectionType,     // Section type
    pub flags: Vec<SectionFlag>, // Section flags
    pub addr: u64,               // Section virtual addr at execution
    pub offset: u64,             // Section file offset
    pub size: u64,               // Section size in bytes
    pub link: u32,               // Link to another section
    pub info: u32,               // Additional section information
    pub addralign: u64,          // Section alignment
    pub entsize: u64,            // Entry size if section holds table
    pub data: &'a [u8],
}
