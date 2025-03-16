use nom::error::{FromExternalError, ParseError};

trait ElfParseError {
    fn as_elf_parser_error(&self) -> ElfError;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ElfError {
    InvalidClass(u8),
    InvalidData(u8),
    InvalidIdentVersion(u8),
    InvalidOSABI(u8),
    InvalidType(u16),
    InvalidMachine(u16),
    InvalidVersion(u32),
    InvalidHeaderSize(u8),
    FileTypeNotElf([u8; 4]),
    Nom(String),
}

impl ElfParseError for ElfError {
    fn as_elf_parser_error(&self) -> ElfError {
        self.clone()
    }
}

impl std::fmt::Display for ElfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::InvalidClass(b) => write!(f, "invalid ELF class: {}", b),
            Self::InvalidData(b) => write!(f, "invalid ELF data: {}", b),
            Self::InvalidIdentVersion(b) => write!(f, "invalid ELF version: {}", b),
            Self::InvalidOSABI(b) => write!(f, "invalid ELF OS/ABI: {}", b),
            Self::InvalidType(b) => write!(f, "invalid ELF type: {}", b),
            Self::InvalidMachine(b) => write!(f, "invalid ELF machine: {}", b),
            Self::InvalidVersion(b) => write!(f, "invalid ELF version: {}", b),
            Self::InvalidHeaderSize(s) => write!(f, "invalid header size: {}", s),
            Self::FileTypeNotElf(ref b) => write!(f, "file type is not ELF: {:?}", b),
            Self::Nom(ref s) => write!(f, "nom error: {}", s),
        }
    }
}

impl std::error::Error for ElfError {}

impl<I: std::fmt::Debug> ParseError<I> for ElfError {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ElfError::Nom(format!("input: {:?}, kind: {:?}", input, kind))
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

// workaround
impl<I, E: ElfParseError> FromExternalError<I, E> for ElfError {
    fn from_external_error(_input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        e.as_elf_parser_error()
    }
}
