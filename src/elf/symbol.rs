#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Type {
    NoType = 0,  // Symbol type is unspecified
    Object = 1,  // Symbol is a data object
    Func = 2,    // Symbol is a code object
    Section = 3, // Symbol associated with a section
    File = 4,    // Symbol's name is file name
    Common = 5,  // Symbol is a common data object
    Tls = 6,     // Symbol is thread-local data object
    Num = 7,     // Number of defined types
    Loos = 10,   // or STT_GNU_IFUNC. Start of OS-specific
    Hios = 12,   // End of OS-specific
    Loproc = 13, // Start of processor-specific
    Hiproc = 15, // End of processor-specific
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Binding {
    Local = 0,   // Local symbol
    Global = 1,  // Global symbol
    Weak = 2,    // Weak symbol
    Loos = 10,   // or GnuUnique. Start of OS-specific
    Hios = 12,   // End of OS-specific
    Loproc = 13, // Start of processor-specific
    Hiproc = 15, // End of processor-specific
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Info {
    pub r#type: Type,
    pub binding: Binding,
}

impl From<Info> for u8 {
    fn from(info: Info) -> Self {
        ((info.binding as u8) << 4) | (info.r#type as u8)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Default = 0,   // Default visibility rules
    Internal = 1,  // Processor-specific hidden class
    Hidden = 2,    // Sym unavailable in other modules
    Protected = 3, // Not preemptible, not exported
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum SymbolIndex {
    Undefined = 0,
    Abs = 0xfff1,
    Common = 0xfff2,
    // TODO
}

impl PartialEq<u16> for SymbolIndex {
    fn eq(&self, other: &u16) -> bool {
        (*self as u16) == *other
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,      // Symbol name
    pub info: Info,        // Symbol type and binding
    pub other: Visibility, // Symbol visibility
    pub shndx: u16,        // Section index
    pub value: u64,        // Symbol value
    pub size: u64,         // Symbol size
}
