use crate::bail_nom_error;

use super::{
    ElfClass, ElfData, ElfHeader, ElfIdent, ElfIdentVersion, ElfMachine, ElfOSABI, ElfType,
    ElfVersion, error::ElfError,
};
use nom::Parser as _;
use nom::combinator::map_res;
use nom::multi::count;
use nom::{
    IResult,
    number::complete::{le_u8, le_u16, le_u32, le_u64},
};

const ELF_MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46]; // 0x7f 'E' 'L' 'F'
const ELF_IDENT_HEADER_SIZE: usize = 16;

type ParseResult<'a, T> = IResult<&'a [u8], T, ElfError>;

fn parse_elf_class(raw: &[u8]) -> ParseResult<ElfClass> {
    map_res(le_u8, |b: u8| ElfClass::try_from(b)).parse(raw)
}

fn parse_elf_data(raw: &[u8]) -> ParseResult<ElfData> {
    map_res(le_u8, |b: u8| ElfData::try_from(b)).parse(raw)
}

fn parse_elf_ident_version(raw: &[u8]) -> ParseResult<ElfIdentVersion> {
    map_res(le_u8, |b: u8| ElfIdentVersion::try_from(b)).parse(raw)
}

fn parse_elf_osabi(raw: &[u8]) -> ParseResult<ElfOSABI> {
    map_res(le_u8, |b: u8| ElfOSABI::try_from(b)).parse(raw)
}

fn parse_ident(raw: &[u8]) -> ParseResult<ElfIdent> {
    let (rest, _) = parse_magic_number(raw)?;

    let (rest, class) = parse_elf_class(rest)?;
    let (rest, data) = parse_elf_data(rest)?;
    let (rest, version) = parse_elf_ident_version(rest)?;
    let (rest, os_abi) = parse_elf_osabi(rest)?;
    let (rest, abi_version) = le_u8(rest)?;
    let (rest, _) = count(le_u8, 7).parse(rest)?;

    let ident = ElfIdent {
        class,
        data,
        os_abi,
        version,
        abi_version,
    };
    Ok((rest, ident))
}

fn parse_type(raw: &[u8]) -> ParseResult<ElfType> {
    map_res(le_u16, |b: u16| ElfType::try_from(b)).parse(raw)
}

fn parse_machine(raw: &[u8]) -> ParseResult<ElfMachine> {
    map_res(le_u16, |b: u16| ElfMachine::try_from(b)).parse(raw)
}

fn parse_version(raw: &[u8]) -> ParseResult<ElfVersion> {
    map_res(le_u32, |b: u32| ElfVersion::try_from(b)).parse(raw)
}

fn parse_magic_number(raw: &[u8]) -> IResult<&[u8], (), ElfError> {
    if raw.len() < 4 {
        bail_nom_error!(ElfError::InvalidHeaderSize(raw.len() as u8));
    }
    if raw[..4] != ELF_MAGIC_NUMBER {
        let input: [u8; 4] = raw[..4].try_into().unwrap();
        bail_nom_error!(ElfError::FileTypeNotElf(input));
    }
    Ok((&raw[4..], ()))
}

pub fn parse_elf_header(raw: &[u8]) -> IResult<&[u8], ElfHeader, ElfError> {
    if raw.len() < ELF_IDENT_HEADER_SIZE {
        bail_nom_error!(ElfError::InvalidHeaderSize(raw.len() as u8));
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
        ElfHeader {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::{ElfMachine, ElfType, ElfVersion};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_elf_class() {
        let input: &[u8] = &[0x02];
        let (rest, class) = parse_elf_class(input).unwrap();
        assert_eq!(rest, &[]);
        assert_eq!(class, ElfClass::Bit64);
    }

    #[test]
    fn should_error_invalid_elf_class() {
        let input: &[u8] = &[0x10];
        let err = parse_elf_class(input).unwrap_err();
        assert_eq!(err, nom::Err::Error(ElfError::InvalidClass(16)));
    }

    #[test]
    fn should_error_invalid_magic_number() {
        let input: &[u8] = &[0u8; 16];
        let err = parse_elf_header(input).unwrap_err();
        assert_eq!(err, nom::Err::Error(ElfError::FileTypeNotElf([0, 0, 0, 0])));
    }

    #[test]
    fn should_parse_elf_header() {
        let raw = include_bytes!("./fixtures/hello.o");
        let (_, ident) = parse_elf_header(raw).unwrap();
        assert_eq!(
            ident,
            ElfHeader {
                ident: ElfIdent {
                    class: ElfClass::Bit64,
                    data: ElfData::Lsb,
                    version: ElfIdentVersion::Current,
                    os_abi: ElfOSABI::SystemV,
                    abi_version: 0,
                },
                r#type: ElfType::Rel,
                machine: ElfMachine::AArch64,
                version: ElfVersion::Current,
                entry_address: 0,
                program_header_offset: 0,
                section_header_offset: 776,
                flags: 0,
                header_size: 64,
                program_header_size: 0,
                program_header_num: 0,
                section_header_size: 64,
                section_header_num: 13,
                section_header_string_table_idx: 12,
            }
        );
    }
}
