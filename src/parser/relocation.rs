use nom::{
    Parser as _,
    combinator::map_res,
    multi::count,
    number::complete::{le_i64, le_u64},
};

use super::{ParseResult, error::ParseError};
use crate::elf::{
    relocation::{Info, RelocationAddend, RelocationType},
    section,
};

impl TryFrom<u32> for RelocationType {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            274 => Ok(Self::Aarch64AdrPrelLo21),
            _ => Err(ParseError::InvalidRelocationType(value)),
        }
    }
}

impl TryFrom<u64> for Info {
    type Error = ParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // low 32 bits is the relocation type
        let r#type = RelocationType::try_from((value & 0xffffffff) as u32)?;
        // high 32 bits is the symbol table index
        let symbol_index = (value >> 32) as u32;
        Ok(Info {
            r#type,
            symbol_index,
        })
    }
}

fn parse_info(raw: &[u8]) -> ParseResult<Info> {
    map_res(le_u64, Info::try_from).parse(raw)
}

pub fn parse<'a>(
    raw: &'a [u8],
    section_headers: &'a [section::Header],
) -> ParseResult<'a, Vec<RelocationAddend>> {
    let Some(header) = section_headers
        .iter()
        .find(|&s| s.r#type == section::SectionType::Rela)
    else {
        return Ok((raw, vec![]));
    };

    let start = header.offset as usize;
    let end = (header.offset + header.size) as usize;
    let entry_count = (header.size / header.entsize) as usize;

    let (rest, relocations) = count(
        |raw| {
            let (rest, offset) = le_u64(raw)?;
            let (rest, info) = parse_info(rest)?;
            let (rest, addend) = le_i64(rest)?;

            let relocation = RelocationAddend {
                offset,
                info,
                addend,
            };

            Ok((rest, relocation))
        },
        entry_count,
    )
    .parse(&raw[start..end])?;

    Ok((rest, relocations))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::elf::relocation::{Info, RelocationAddend, RelocationType};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_info() {
        let value: u64 = 0x00000010_00000112;
        let raw = value.to_le_bytes();
        let info = super::parse_info(&raw).unwrap().1;
        assert_eq!(
            info,
            Info {
                r#type: RelocationType::Aarch64AdrPrelLo21,
                symbol_index: 16,
            }
        );
    }

    #[test]
    fn should_parse_relocation() {
        let raw = include_bytes!("./fixtures/main.o");
        let (_, header) = crate::parser::header::parse(raw).unwrap();
        let (_, section_headers) = crate::parser::section::parse_header(
            raw,
            header.shoff as usize,
            header.shstrndx as usize,
            header.shnum as usize,
        )
        .unwrap();

        let reloc = parse(raw, &section_headers).unwrap().1;
        assert_eq!(
            reloc,
            vec![RelocationAddend {
                offset: 0,
                info: Info {
                    r#type: RelocationType::Aarch64AdrPrelLo21,
                    symbol_index: 9,
                },
                addend: 0,
            }]
        );
    }
}
