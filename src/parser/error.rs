use thiserror::Error;

trait ELFParseError {
    fn as_parse_error(&self) -> ParseError;
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    // ELF header
    #[error("Invalid ELF class: {0}")]
    InvalidClass(u8),
    #[error("Invalid ELF data encoding: {0}")]
    InvalidData(u8),
    #[error("Invalid ELF identification version: {0}")]
    InvalidIdentVersion(u8),
    #[error("Invalid OS ABI: {0}")]
    InvalidOSABI(u8),
    #[error("Invalid ELF type: {0}")]
    InvalidType(u16),
    #[error("Invalid machine type: {0}")]
    InvalidMachine(u16),
    #[error("Invalid ELF version: {0}")]
    InvalidVersion(u32),
    #[error("Invalid ELF header size: {0}")]
    InvalidHeaderSize(u8),
    #[error("File is not an ELF file (magic: {0:?})")]
    FileTypeNotELF([u8; 4]),
    // Section Header
    #[error("Invalid section type: {0}")]
    InvalidSectionType(u32),
    #[error("Invalid section flags: {0}")]
    InvalidSectionFlags(u64),
    // Symbol Table
    #[error("Invalid symbol visibility: {0}")]
    InvalidVisibility(u8),
    #[error("Invalid symbol type: {0}")]
    InvalidSymbolType(u8),
    #[error("Invalid symbol binding: {0}")]
    InvalidSymbolBinding(u8),
    // Relocation Addend
    #[error("Invalid relocation type: {0}")]
    InvalidRelocationType(u32),
    #[error("Parser error: {0}")]
    Nom(String),
}

impl ELFParseError for ParseError {
    fn as_parse_error(&self) -> ParseError {
        self.clone()
    }
}

impl<I: std::fmt::Debug> nom::error::ParseError<I> for ParseError {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ParseError::Nom(format!("input: {:?}, kind: {:?}", input, kind))
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

// workaround
impl<I, E: ELFParseError> nom::error::FromExternalError<I, E> for ParseError {
    fn from_external_error(_input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        e.as_parse_error()
    }
}
