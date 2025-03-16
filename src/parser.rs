pub mod error;
pub mod header;
pub mod section;

use crate::elf::Elf;
use crate::parser::error::ParseError;
use nom::IResult;

pub type ParseResult<'a, T> = IResult<&'a [u8], T, ParseError>;

pub fn parse_elf(raw: &[u8]) -> ParseResult<Elf> {
    let (rest, header) = header::parse(raw)?;
    Ok((rest, Elf { header }))
}
