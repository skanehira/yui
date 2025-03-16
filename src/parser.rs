pub mod error;
pub mod header;

use crate::elf::Elf;
use crate::parser::error::ParseError;
use nom::IResult;

pub fn parse_elf(raw: &[u8]) -> IResult<&[u8], Elf, ParseError> {
    let (rest, header) = header::parse(raw)?;
    Ok((rest, Elf { header }))
}
