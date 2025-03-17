pub mod error;
pub mod header;
pub mod section;

use crate::elf::Elf;
use crate::parser::error::ParseError;
use nom::IResult;

pub type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

pub fn parse_elf(raw: &[u8]) -> ParseResult<Elf> {
    let (_, header) = header::parse(raw)?;
    let section_headers = if header.shnum == 0 {
        vec![]
    } else {
        let (_, section_headers) = section::parse_header_table(
            raw,
            header.shoff as usize,
            header.shstrndx as usize,
            header.shnum as usize,
        )?;
        section_headers
    };

    Ok((
        &[],
        Elf {
            header,
            section_headers,
        },
    ))
}
