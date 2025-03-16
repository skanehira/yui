#[derive(Default, Debug, PartialEq, Eq)]
pub enum Class {
    #[default]
    None = 0,
    Bit32 = 1,
    Bit64 = 2,
    Num = 3,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Data {
    #[default]
    None = 0, // unknown
    Lsb = 1, // little-endian.
    Msb = 2, // big-endian.
    Num = 3,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum IdentVersion {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

#[derive(Default, Debug, PartialEq, Eq)]
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

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Type {
    #[default]
    None = 0, // No file type
    Rel = 1,         // Relocatable file
    Exec = 2,        // Executable file
    Dyn = 3,         // Shared object file
    Core = 4,        // Core file
    Num = 5,         // Number of defined types
    Loos = 0xfe00,   // OS-specific range start
    Hios = 0xfeff,   // OS-specific range end
    Loproc = 0xff00, // Processor-specific range start
    Hiproc = 0xffff, // Processor-specific range end
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Machine {
    #[default]
    None = 0,
    X86_64 = 62,
    AArch64 = 183,
    RiscV = 243,
    // TODO: Add more architectures
    Num = 253,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Version {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Header {
    pub ident: Ident,     // Magic number and other info
    pub r#type: Type,     // Object file type
    pub machine: Machine, // Architecture
    pub version: Version, // Object file version
    pub entry: u64,       // Entry point virtual address
    pub phoff: u64,       // Program header table file offset
    pub shoff: u64,       // Section header table file offset
    pub flags: u32,       // Processor-specific flags
    pub ehsize: u16,      // ELF header size in bytes
    pub phentsize: u16,   // Program header table entry size
    pub phnum: u16,       // Program header table entry count
    pub shentsize: u16,   // Section header table entry size
    pub shnum: u16,       // Section header table entry count
    pub shstrndx: u16,    // Section header string table index
}
