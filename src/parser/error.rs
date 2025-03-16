trait ElfParseError {
    fn as_parse_error(&self) -> ParseError;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
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

impl ElfParseError for ParseError {
    fn as_parse_error(&self) -> ParseError {
        self.clone()
    }
}

impl std::fmt::Display for ParseError {
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

impl std::error::Error for ParseError {}

impl<I: std::fmt::Debug> nom::error::ParseError<I> for ParseError {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ParseError::Nom(format!("input: {:?}, kind: {:?}", input, kind))
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

// workaround
impl<I, E: ElfParseError> nom::error::FromExternalError<I, E> for ParseError {
    fn from_external_error(_input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        e.as_parse_error()
    }
}
