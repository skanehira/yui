use error::ElfError;

pub mod error;
pub mod parser;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfClass {
    #[default]
    None = 0,
    Bit32 = 1,
    Bit64 = 2,
    Num = 3,
}

impl TryFrom<u8> for ElfClass {
    type Error = ElfError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Bit32),
            2 => Ok(Self::Bit64),
            3 => Ok(Self::Num),
            _ => Err(ElfError::InvalidClass(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfData {
    #[default]
    None = 0, // unknown
    Lsb = 1, // little-endian.
    Msb = 2, // big-endian.
    Num = 3,
}

impl TryFrom<u8> for ElfData {
    type Error = ElfError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Lsb),
            2 => Ok(Self::Msb),
            _ => Err(ElfError::InvalidData(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfIdentVersion {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

impl TryFrom<u8> for ElfIdentVersion {
    type Error = ElfError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Current),
            2 => Ok(Self::Num),
            _ => Err(ElfError::InvalidIdentVersion(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfOSABI {
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

impl TryFrom<u8> for ElfOSABI {
    type Error = ElfError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::SystemV),
            1 => Ok(Self::HpUx),
            2 => Ok(Self::NetBsd),
            3 => Ok(Self::Gnu),
            6 => Ok(Self::Solaris),
            7 => Ok(Self::Aix),
            8 => Ok(Self::Irix),
            9 => Ok(Self::FreeBsd),
            10 => Ok(Self::Tru64),
            11 => Ok(Self::Modesto),
            12 => Ok(Self::OpenBsd),
            64 => Ok(Self::ArmAeabi),
            97 => Ok(Self::Arm),
            255 => Ok(Self::Standalone),
            _ => Err(ElfError::InvalidOSABI(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ElfIdent {
    pub class: ElfClass,
    pub data: ElfData,
    pub version: ElfIdentVersion,
    pub os_abi: ElfOSABI,
    pub abi_version: u8,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfType {
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

impl TryFrom<u16> for ElfType {
    type Error = ElfError;
    fn try_from(b: u16) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Rel),
            2 => Ok(Self::Exec),
            3 => Ok(Self::Dyn),
            4 => Ok(Self::Core),
            5 => Ok(Self::Num),
            0xfe00 => Ok(Self::Loos),
            0xfeff => Ok(Self::Hios),
            0xff00 => Ok(Self::Loproc),
            0xffff => Ok(Self::Hiproc),
            _ => Err(ElfError::InvalidType(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfMachine {
    #[default]
    None = 0,
    X86_64 = 62,
    AArch64 = 183,
    RiscV = 243,
    // TODO: Add more architectures
    Num = 253,
}

impl TryFrom<u16> for ElfMachine {
    type Error = ElfError;
    fn try_from(b: u16) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            62 => Ok(Self::X86_64),
            183 => Ok(Self::AArch64),
            243 => Ok(Self::RiscV),
            253 => Ok(Self::Num),
            _ => Err(ElfError::InvalidMachine(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ElfVersion {
    #[default]
    None = 0,
    Current = 1,
    Num = 2,
}

impl TryFrom<u32> for ElfVersion {
    type Error = ElfError;
    fn try_from(b: u32) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Current),
            2 => Ok(Self::Num),
            _ => Err(ElfError::InvalidVersion(b)),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ElfHeader {
    pub ident: ElfIdent,
    pub r#type: ElfType,
    pub machine: ElfMachine,
    pub version: ElfVersion,
    pub entry_address: u64,
    pub program_header_offset: u64,
    pub section_header_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_size: u16,
    pub program_header_num: u16,
    pub section_header_size: u16,
    pub section_header_num: u16,
    pub section_header_string_table_idx: u16,
}

pub struct ElfSection {
    pub name: String,
}

pub struct Elf {
    pub ident: ElfIdent,
    pub sections: Vec<ElfSection>,
}
