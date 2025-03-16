use crate::bail_nom_error;
use crate::elf::header::{Class, Data, Header, Ident, IdentVersion, Machine, OSABI, Type, Version};
use crate::parser::error::ParseError;
use nom::Parser as _;
use nom::combinator::map_res;
use nom::multi::count;
use nom::{
    IResult,
    number::complete::{le_u8, le_u16, le_u32, le_u64},
};

const ELF_MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46]; // 0x7f 'E' 'L' 'F'
const ELF_IDENT_HEADER_SIZE: usize = 16;

type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

impl TryFrom<u8> for Class {
    type Error = ParseError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Bit32),
            2 => Ok(Self::Bit64),
            3 => Ok(Self::Num),
            _ => Err(ParseError::InvalidClass(b)),
        }
    }
}

impl TryFrom<u8> for Data {
    type Error = ParseError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Lsb),
            2 => Ok(Self::Msb),
            _ => Err(ParseError::InvalidData(b)),
        }
    }
}

impl TryFrom<u8> for IdentVersion {
    type Error = ParseError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Current),
            2 => Ok(Self::Num),
            _ => Err(ParseError::InvalidIdentVersion(b)),
        }
    }
}

impl TryFrom<u8> for OSABI {
    type Error = ParseError;
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
            _ => Err(ParseError::InvalidOSABI(b)),
        }
    }
}

impl TryFrom<u16> for Type {
    type Error = ParseError;
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
            _ => Err(ParseError::InvalidType(b)),
        }
    }
}

impl TryFrom<u16> for Machine {
    type Error = ParseError;
    fn try_from(b: u16) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            62 => Ok(Self::X86_64),
            183 => Ok(Self::AArch64),
            243 => Ok(Self::RiscV),
            253 => Ok(Self::Num),
            _ => Err(ParseError::InvalidMachine(b)),
        }
    }
}

impl TryFrom<u32> for Version {
    type Error = ParseError;
    fn try_from(b: u32) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::None),
            1 => Ok(Self::Current),
            2 => Ok(Self::Num),
            _ => Err(ParseError::InvalidVersion(b)),
        }
    }
}

fn parse_elf_class(raw: &[u8]) -> ParseResult<Class> {
    map_res(le_u8, |b: u8| Class::try_from(b)).parse(raw)
}

fn parse_elf_data(raw: &[u8]) -> ParseResult<Data> {
    map_res(le_u8, |b: u8| Data::try_from(b)).parse(raw)
}

fn parse_elf_ident_version(raw: &[u8]) -> ParseResult<IdentVersion> {
    map_res(le_u8, |b: u8| IdentVersion::try_from(b)).parse(raw)
}

fn parse_elf_osabi(raw: &[u8]) -> ParseResult<OSABI> {
    map_res(le_u8, |b: u8| OSABI::try_from(b)).parse(raw)
}

fn parse_ident(raw: &[u8]) -> ParseResult<Ident> {
    let (rest, _) = parse_magic_number(raw)?;

    let (rest, class) = parse_elf_class(rest)?;
    let (rest, data) = parse_elf_data(rest)?;
    let (rest, version) = parse_elf_ident_version(rest)?;
    let (rest, os_abi) = parse_elf_osabi(rest)?;
    let (rest, abi_version) = le_u8(rest)?;
    let (rest, _) = count(le_u8, 7).parse(rest)?;

    let ident = Ident {
        class,
        data,
        os_abi,
        version,
        abi_version,
    };
    Ok((rest, ident))
}

fn parse_type(raw: &[u8]) -> ParseResult<Type> {
    map_res(le_u16, |b: u16| Type::try_from(b)).parse(raw)
}

fn parse_machine(raw: &[u8]) -> ParseResult<Machine> {
    map_res(le_u16, |b: u16| Machine::try_from(b)).parse(raw)
}

fn parse_version(raw: &[u8]) -> ParseResult<Version> {
    map_res(le_u32, |b: u32| Version::try_from(b)).parse(raw)
}

fn parse_magic_number(raw: &[u8]) -> IResult<&[u8], (), ParseError> {
    if raw.len() < 4 {
        bail_nom_error!(ParseError::InvalidHeaderSize(raw.len() as u8));
    }
    if raw[..4] != ELF_MAGIC_NUMBER {
        let input: [u8; 4] = raw[..4].try_into().unwrap();
        bail_nom_error!(ParseError::FileTypeNotElf(input));
    }
    Ok((&raw[4..], ()))
}

pub fn parse(raw: &[u8]) -> IResult<&[u8], Header, ParseError> {
    if raw.len() < ELF_IDENT_HEADER_SIZE {
        bail_nom_error!(ParseError::InvalidHeaderSize(raw.len() as u8));
    }
    let (rest, ident) = parse_ident(raw)?;
    let (rest, r#type) = parse_type(rest)?;
    let (rest, machine) = parse_machine(rest)?;
    let (rest, version) = parse_version(rest)?;
    let (rest, entry_address) = le_u64(rest)?;
    let (rest, program_header_offset) = le_u64(rest)?;
    let (rest, section_header_offset) = le_u64(rest)?;
    let (rest, flags) = le_u32(rest)?;
    let (rest, header_size) = le_u16(rest)?;
    let (rest, program_header_size) = le_u16(rest)?;
    let (rest, program_header_num) = le_u16(rest)?;
    let (rest, section_header_size) = le_u16(rest)?;
    let (rest, section_header_num) = le_u16(rest)?;
    let (rest, section_header_string_table_idx) = le_u16(rest)?;

    Ok((
        rest,
        Header {
            ident,
            r#type,
            machine,
            version,
            entry: entry_address,
            phoff: program_header_offset,
            shoff: section_header_offset,
            flags,
            ehsize: header_size,
            phentsize: program_header_size,
            phnum: program_header_num,
            shentsize: section_header_size,
            shnum: section_header_num,
            shstrndx: section_header_string_table_idx,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::header::{Machine, Type, Version};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_elf_class() {
        let input: &[u8] = &[0x02];
        let (rest, class) = parse_elf_class(input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(class, Class::Bit64);
    }

    #[test]
    fn should_error_invalid_elf_class() {
        let input: &[u8] = &[0x10];
        let err = parse_elf_class(input).unwrap_err();
        assert_eq!(err, nom::Err::Error(ParseError::InvalidClass(16)));
    }

    #[test]
    fn should_error_invalid_magic_number() {
        let input: &[u8] = &[0u8; 16];
        let err = parse(input).unwrap_err();
        assert_eq!(
            err,
            nom::Err::Error(ParseError::FileTypeNotElf([0, 0, 0, 0]))
        );
    }

    #[test]
    fn should_parse_elf_header() {
        let raw = include_bytes!("./fixtures/sub.o");
        let (_, ident) = parse(raw).unwrap();
        assert_eq!(
            ident,
            Header {
                ident: Ident {
                    class: Class::Bit64,
                    data: Data::Lsb,
                    version: IdentVersion::Current,
                    os_abi: OSABI::SystemV,
                    abi_version: 0,
                },
                r#type: Type::Rel,
                machine: Machine::AArch64,
                version: Version::Current,
                entry: 0,
                phoff: 0,
                shoff: 416,
                flags: 0,
                ehsize: 64,
                phentsize: 0,
                phnum: 0,
                shentsize: 64,
                shnum: 9,
                shstrndx: 8,
            }
        );
    }
}
