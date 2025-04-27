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

/// Parses the section headers from the raw ELF file data.
///
/// # Arguments
///
/// * `raw` - A byte slice containing the raw ELF file data.
/// * `shoff` - The offset in the file where the section header table begins.
/// * `shstrndx` - The index of the section header string table in the section header table.
/// * `shnum` - The number of section headers in the section header table.
///
/// # Returns
///
/// A `ParseResult` containing a vector of parsed `Header` structures or a `ParseError` if parsing fails.
///
/// # Notes
///
/// If `shnum` is 0, the function returns an empty vector of headers. The function uses a parser combinator
/// to iterate through the section headers and extract their fields. After parsing, it resolves the section
/// names using the section header string table.
pub fn parse_header(
    raw: &[u8],
    shoff: usize,
    shstrndx: usize,
    shnum: usize,
) -> ParseResult<Vec<Header>> {
    if shnum == 0 {
        return Ok((raw, vec![]));
    }

    count(
        |rest| {
            let (rest, name_idx) = le_u32(rest)?;
            let (rest, r#type) = parse_type(rest)?;
            let (rest, flags) = parse_flags(rest)?;
            let (rest, addr) = le_u64(rest)?;
            let (rest, offset) = le_u64(rest)?;
            let (rest, size) = le_u64(rest)?;
            let (rest, link) = le_u32(rest)?;
            let (rest, info) = le_u32(rest)?;
            let (rest, addralign) = le_u64(rest)?;
            let (rest, entsize) = le_u64(rest)?;
            let data = raw[offset as usize..(offset + size) as usize].to_vec();

            let header = Header {
                name_idx,
                name: "".into(), // to be filled later
                r#type,
                flags,
                addr,
                offset,
                size,
                link,
                info,
                addralign,
                entsize,
                section_raw_data: data,
            };

            Ok((rest, header))
        },
        shnum,
    )
    .parse(&raw[shoff..])
    .map(|(rest, mut headers)| {
        let string_table = &raw[headers[shstrndx].offset as usize..];
        for header in headers.iter_mut() {
            header.name = helper::get_string_by_offset(string_table, header.name_idx as usize);
        }
        (rest, headers)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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

        insta::assert_debug_snapshot!(header_table);
    }
}
