trait ELFParseError {
    fn as_parse_error(&self) -> ParseError;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    // ELF header
    InvalidClass(u8),
    InvalidData(u8),
    InvalidIdentVersion(u8),
    InvalidOSABI(u8),
    InvalidType(u16),
    InvalidMachine(u16),
    InvalidVersion(u32),
    InvalidHeaderSize(u8),
    FileTypeNotELF([u8; 4]),
    // Section Header
    InvalidSectionType(u32),
    InvalidSectionFlags(u64),
    // Symbol Table
    InvalidVisibility(u8),
    InvalidSymbolType(u8),
    InvalidSymbolBinding(u8),
    // Relocation Addend
    InvalidRelocationType(u32),
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
