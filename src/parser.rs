mod helper;

pub mod error;
pub mod header;
pub mod relocation;
pub mod section;
pub mod symbol;

use crate::elf::Elf;
use crate::parser::error::ParseError;
use nom::IResult;

pub type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

pub fn parse_elf(raw: &[u8]) -> ParseResult<Elf> {
    let header = header::parse(raw)?.1;
    let section_headers = if header.shnum == 0 {
        vec![]
    } else {
        section::parse_header(
            raw,
            header.shoff as usize,
            header.shstrndx as usize,
            header.shnum as usize,
        )?
        .1
    };

    let symbols = symbol::parse(raw, &section_headers).unwrap().1;

    let relocation = relocation::parse(raw, &section_headers).unwrap().1;

    Ok((
        &[],
        Elf {
            header,
            section_headers,
            symbols,
            relocation,
        },
    ))
}
