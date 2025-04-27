mod helper;

pub mod error;
pub mod header;
pub mod relocation;
pub mod section;
pub mod symbol;

use crate::elf::ELF;
use crate::parser::error::ParseError;
use nom::IResult;

/// A type alias for the result of parsing operations, which includes the remaining input,
/// the parsed value, and any potential parsing error.
pub type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

/// Parses the raw bytes of an ELF file into an `ELF` structure.
///
/// # Arguments
///
/// * `raw` - A byte slice containing the raw ELF file data.
///
/// # Returns
///
/// A `ParseResult` containing the parsed `ELF` structure or a `ParseError` if parsing fails.
pub fn parse_elf(raw: &[u8]) -> ParseResult<ELF> {
    let header = header::parse(raw)?.1;

    let section_headers = section::parse_header(
        raw,
        header.shoff as usize,
        header.shstrndx as usize,
        header.shnum as usize,
    )?
    .1;

    let symbols = symbol::parse(raw, &section_headers)?.1;

    let relocations = relocation::parse(&section_headers)?.1;

    Ok((
        &[],
        ELF {
            header,
            section_headers,
            symbols,
            relocations,
        },
    ))
}
