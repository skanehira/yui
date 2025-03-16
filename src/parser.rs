pub mod error;
pub mod header;

use crate::elf::Elf;
use crate::parser::error::ParseError;
use header::parse;

pub fn parse_elf(raw: &[u8]) -> Result<Elf, ParseError> {
    let header = parse(raw)
        .map(|(_, header)| header)
        .map_err(|e| match e {
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
            nom::Err::Incomplete(e) => ParseError::Nom(format!("incomplete: {:?}", e)),
        })?;
    Ok(Elf { header })
}
