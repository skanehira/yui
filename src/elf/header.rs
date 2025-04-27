#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Class {
    #[default]
    None = 0,
    Bit32 = 1,
    Bit64 = 2,
    Num = 3,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Data {
    #[default]
    None = 0, // unknown
    Lsb = 1, // little-endian.
    Msb = 2, // big-endian.
    Num = 3,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum IdentVersion {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum OSABI {
    #[default]
    SystemV = 0, // or none
    HpUx = 1,
    NetBsd = 2,
    Gnu = 3, // or linux
    Solaris = 6,
    Aix = 7,
    Irix = 8,
    FreeBsd = 9,
    Tru64 = 10,
    Modesto = 11,
    OpenBsd = 12,
    ArmAeabi = 64,
    Arm = 97,
    Standalone = 255,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Ident {
    pub class: Class,
    pub data: Data,
    pub version: IdentVersion,
    pub os_abi: OSABI,
    pub abi_version: u8,
}

/// Represents the type of an ELF file.
///
/// This enum defines various types of ELF files, such as relocatable files,
/// executable files, shared object files, and core files. It also includes
/// ranges for OS-specific and processor-specific types.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Type {
    /// No file type.
    #[default]
    None = 0,
    /// Relocatable file.
    Rel = 1,
    /// Executable file.
    Exec = 2,
    /// Shared object file.
    Dyn = 3,
    /// Core file.
    Core = 4,
    /// Number of defined types.
    Num = 5,
    /// OS-specific range start.
    Loos = 0xfe00,
    /// OS-specific range end.
    Hios = 0xfeff,
    /// Processor-specific range start.
    Loproc = 0xff00,
    /// Processor-specific range end.
    Hiproc = 0xffff,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Machine {
    #[default]
    None = 0,
    X86_64 = 62,
    AArch64 = 183,
    RiscV = 243,
    // TODO: Add more architectures
    Num = 253,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Version {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

/// Represents the ELF (Executable and Linkable Format) file header.
///
/// The ELF header contains metadata about the ELF file, such as its type, architecture,
/// entry point, and offsets to other important structures like the program and section headers.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Header {
    /// Identification information, including the magic number and file class.
    pub ident: Ident,
    /// The type of the ELF file (e.g., relocatable, executable, shared object).
    pub r#type: Type,
    /// The target architecture for the ELF file (e.g., x86, ARM).
    pub machine: Machine,
    /// The version of the ELF file format.
    pub version: Version,
    /// The virtual address of the entry point for the program.
    pub entry: u64,
    /// The file offset to the program header table.
    pub phoff: u64,
    /// The file offset to the section header table.
    pub shoff: u64,
    /// Processor-specific flags.
    pub flags: u32,
    /// The size of the ELF header in bytes.
    pub ehsize: u16,
    /// The size of each entry in the program header table.
    pub phentsize: u16,
    /// The number of entries in the program header table.
    pub phnum: u16,
    /// The size of each entry in the section header table.
    pub shentsize: u16,
    /// The number of entries in the section header table.
    pub shnum: u16,
    /// The index of the section header string table.
    pub shstrndx: u16,
}
