use super::Elf;
use super::{
    Class, Data, Header, Ident, IdentVersion, Machine, OSABI, Type, Version, error::ParseError,
};
use crate::bail_nom_error;
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

fn parse_elf_header(raw: &[u8]) -> IResult<&[u8], Header, ParseError> {
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
            entry_address,
            program_header_offset,
            section_header_offset,
            flags,
            header_size,
            program_header_size,
            program_header_num,
            section_header_size,
            section_header_num,
            section_header_string_table_idx,
        },
    ))
}

pub fn parse_elf(raw: &[u8]) -> Result<Elf, ParseError> {
    let header = parse_elf_header(raw)
        .map(|(_, header)| header)
        .map_err(|e| match e {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(e) => ParseError::Nom(format!("incomplete: {:?}", e)),
        })?;
    Ok(Elf {
        header,
        sections: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::{Machine, Type, Version};
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
        let err = parse_elf_header(input).unwrap_err();
        assert_eq!(
            err,
            nom::Err::Error(ParseError::FileTypeNotElf([0, 0, 0, 0]))
        );
    }

    #[test]
    fn should_parse_elf_header() {
        let raw = include_bytes!("./fixtures/sub.o");
        let (_, ident) = parse_elf_header(raw).unwrap();
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
                entry_address: 0,
                program_header_offset: 0,
                section_header_offset: 416,
                flags: 0,
                header_size: 64,
                program_header_size: 0,
                program_header_num: 0,
                section_header_size: 64,
                section_header_num: 9,
                section_header_string_table_idx: 8,
            }
        );
    }
}
