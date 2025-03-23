use crate::{
    elf::section::{Header, SectionFlag, SectionType},
    parser::error::ParseError,
};
use nom::{
    Parser as _,
    combinator::map_res,
    multi::count,
    number::complete::{le_u32, le_u64},
};

use super::{ParseResult, helper};

impl TryFrom<u32> for SectionType {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Null),
            1 => Ok(Self::ProgBits),
            2 => Ok(Self::SymTab),
            3 => Ok(Self::StrTab),
            4 => Ok(Self::Rela),
            5 => Ok(Self::Hash),
            6 => Ok(Self::Dynamic),
            7 => Ok(Self::Note),
            8 => Ok(Self::NoBits),
            9 => Ok(Self::Rel),
            10 => Ok(Self::ShLib),
            11 => Ok(Self::DynSym),
            14 => Ok(Self::InitArray),
            15 => Ok(Self::FiniArray),
            16 => Ok(Self::PreInitArray),
            17 => Ok(Self::Group),
            18 => Ok(Self::SymTabShndx),
            0x6ffffffa => Ok(Self::GnuVerdef),
            0x6ffffffb => Ok(Self::GnuVerneed),
            0x6ffffffc => Ok(Self::GnuVersym),
            _ => Err(ParseError::InvalidSectionType(value)),
        }
    }
}

fn parse_type(raw: &[u8]) -> ParseResult<SectionType> {
    map_res(le_u32, SectionType::try_from).parse(raw)
}

fn parse_flags(raw: &[u8]) -> ParseResult<Vec<SectionFlag>> {
    map_res(le_u64, |mask| -> Result<Vec<SectionFlag>, ParseError> {
        let flag_variants = [
            SectionFlag::Write,
            SectionFlag::Alloc,
            SectionFlag::ExecInstr,
            SectionFlag::Merge,
            SectionFlag::Strings,
            SectionFlag::InfoLink,
            SectionFlag::LinkOrder,
            SectionFlag::Group,
            SectionFlag::Tls,
            SectionFlag::Compressed,
        ];

        if mask == 0 {
            return Ok(vec![]);
        }

        let flags = flag_variants
            .into_iter()
            .filter(|flag| mask & (*flag as u64) != 0)
            .collect();

        Ok(flags)
    })
    .parse(raw)
}

pub fn parse_header(
    raw: &[u8],
    shoff: usize,
    shstrndx: usize,
    shnum: usize,
) -> ParseResult<Vec<Header>> {
    count(
        |raw| {
            let (rest, name_idx) = le_u32(raw)?;
            let (rest, r#type) = parse_type(rest)?;
            let (rest, flags) = parse_flags(rest)?;
            let (rest, addr) = le_u64(rest)?;
            let (rest, offset) = le_u64(rest)?;
            let (rest, size) = le_u64(rest)?;
            let (rest, link) = le_u32(rest)?;
            let (rest, info) = le_u32(rest)?;
            let (rest, addralign) = le_u64(rest)?;
            let (rest, entsize) = le_u64(rest)?;

            let header = Header {
                name_idx,
                name: "".into(),
                r#type,
                flags,
                addr,
                offset,
                size,
                link,
                info,
                addralign,
                entsize,
            };

            Ok((rest, header))
        },
        shnum,
    )
    .parse(&raw[shoff..])
    .and_then(|(rest, mut headers)| {
        let string_table = &raw[headers[shstrndx].offset as usize..];
        for header in headers.iter_mut() {
            header.name = helper::get_string_by_offset(string_table, header.name_idx as usize)?.1;
        }
        Ok((rest, headers))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_section_header_table() {
        let raw = include_bytes!("./fixtures/sub.o");
        let (_, header) = crate::parser::header::parse(raw).unwrap();

        let (_, header_table) = parse_header(
            raw,
            header.shoff as usize,
            header.shstrndx as usize,
            header.shnum as usize,
        )
        .unwrap();

        let want = vec![
            Header {
                name_idx: 0,
                name: "".into(),
                r#type: SectionType::Null,
                flags: vec![],
                addr: 0,
                offset: 0,
                size: 0,
                link: 0,
                info: 0,
                addralign: 0,
                entsize: 0,
            },
            Header {
                name_idx: 27,
                name: ".text".into(),
                r#type: SectionType::ProgBits,
                flags: vec![SectionFlag::Alloc, SectionFlag::ExecInstr],
                addr: 0,
                offset: 64,
                size: 0,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 0,
            },
            Header {
                name_idx: 33,
                name: ".data".into(),
                r#type: SectionType::ProgBits,
                flags: vec![SectionFlag::Write, SectionFlag::Alloc],
                addr: 0,
                offset: 64,
                size: 4,
                link: 0,
                info: 0,
                addralign: 4,
                entsize: 0,
            },
            Header {
                name_idx: 39,
                name: ".bss".into(),
                r#type: SectionType::NoBits,
                flags: vec![SectionFlag::Write, SectionFlag::Alloc],
                addr: 0,
                offset: 68,
                size: 0,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 0,
            },
            Header {
                name_idx: 44,
                name: ".comment".into(),
                r#type: SectionType::ProgBits,
                flags: vec![SectionFlag::Merge, SectionFlag::Strings],
                addr: 0,
                offset: 68,
                size: 40,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 1,
            },
            Header {
                name_idx: 53,
                name: ".note.GNU-stack".into(),
                r#type: SectionType::ProgBits,
                flags: vec![],
                addr: 0,
                offset: 108,
                size: 0,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 0,
            },
            Header {
                name_idx: 1,
                name: ".symtab".into(),
                r#type: SectionType::SymTab,
                flags: vec![],
                addr: 0,
                offset: 112,
                size: 216,
                link: 7,
                info: 8,
                addralign: 8,
                entsize: 24,
            },
            Header {
                name_idx: 9,
                name: ".strtab".into(),
                r#type: SectionType::StrTab,
                flags: vec![],
                addr: 0,
                offset: 328,
                size: 12,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 0,
            },
            Header {
                name_idx: 17,
                name: ".shstrtab".into(),
                r#type: SectionType::StrTab,
                flags: vec![],
                addr: 0,
                offset: 340,
                size: 69,
                link: 0,
                info: 0,
                addralign: 1,
                entsize: 0,
            },
        ];
        assert_eq!(header_table, want);
    }
}
