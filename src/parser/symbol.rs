use super::{ParseResult, error::ParseError, helper};
use crate::elf::{
    section::{Header, SectionType},
    symbol::{Binding, Info, Symbol, Type, Visibility},
};
use nom::{
    Parser as _,
    combinator::map_res,
    multi::count,
    number::complete::{le_u8, le_u16, le_u32, le_u64},
};

impl TryFrom<u8> for Visibility {
    type Error = ParseError;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0 => Ok(Self::Default),
            1 => Ok(Self::Internal),
            2 => Ok(Self::Hidden),
            3 => Ok(Self::Protected),
            _ => Err(ParseError::InvalidVisibility(b)),
        }
    }
}

impl TryFrom<u8> for Info {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // high 4 bites: Binding
        // 0b00100001
        // ----------
        // 0b00000010
        let binding = match value >> 4 {
            0 => Binding::Local,
            1 => Binding::Global,
            2 => Binding::Weak,
            10 => Binding::Loos,
            12 => Binding::Hios,
            13 => Binding::Loproc,
            15 => Binding::Hiproc,
            _ => return Err(ParseError::InvalidSymbolBinding(value)),
        };

        // low 4 bites: Type
        // 0b00000001
        // 0b00001111
        // ----------
        // 0b00000001
        let r#type = match value & 0xf {
            0 => Type::NoType,
            1 => Type::Object,
            2 => Type::Func,
            3 => Type::Section,
            4 => Type::File,
            5 => Type::Common,
            6 => Type::Tls,
            7 => Type::Num,
            10 => Type::Loos,
            12 => Type::Hios,
            13 => Type::Loproc,
            15 => Type::Hiproc,
            _ => return Err(ParseError::InvalidSymbolType(value)),
        };

        Ok(Self { r#type, binding })
    }
}

fn parse_info(raw: &[u8]) -> ParseResult<Info> {
    map_res(le_u8, Info::try_from).parse(raw)
}

pub fn parse<'a>(raw: &'a [u8], section_headers: &'a [Header]) -> ParseResult<'a, Vec<Symbol>> {
    let Some(symbol_header) = section_headers
        .iter()
        .find(|header| header.r#type == SectionType::SymTab)
    else {
        return Ok((&[], vec![]));
    };

    // will be .strtab
    let string_table = &raw[section_headers[symbol_header.link as usize].offset as usize..];

    let entry_count = (symbol_header.size / symbol_header.entsize) as usize;

    let start = symbol_header.offset as usize;
    let end = (symbol_header.offset + symbol_header.size) as usize;

    let (rest, symbols) = count(
        |raw| {
            let (rest, name_idx) = le_u32(raw)?;
            let (rest, info) = parse_info(rest)?;
            let (rest, other) = map_res(le_u8, Visibility::try_from).parse(rest)?;
            let (rest, shndx) = le_u16(rest)?;
            let (rest, value) = le_u64(rest)?;
            let (rest, size) = le_u64(rest)?;

            let name = helper::get_string_by_offset(string_table, name_idx as usize)?.1;
            let symbol = Symbol {
                name,
                info,
                other,
                shndx,
                value,
                size,
            };

            Ok((rest, symbol))
        },
        entry_count,
    )
    .parse(&raw[start..end])?;

    Ok((rest, symbols))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::elf::symbol::{Binding, Info, Symbol, Type, Visibility};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_info_ok() {
        // high: 1 = global
        // low: 2 = func
        let raw = 0b00010010;
        let info = Info::try_from(raw).unwrap();
        assert_eq!(
            info,
            Info {
                r#type: Type::Func,
                binding: Binding::Global,
            }
        );
    }

    #[test]
    fn parse_symbol_ok() {
        let raw = include_bytes!("./fixtures/sub.o");
        let (_, header) = crate::parser::header::parse(raw).unwrap();
        let (_, section_headers) = crate::parser::section::parse_header(
            raw,
            header.shoff as usize,
            header.shstrndx as usize,
            header.shnum as usize,
        )
        .unwrap();

        let symbols = parse(raw, &section_headers).unwrap().1;
        let want = [
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::NoType,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 0,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "sub.c".into(),
                info: Info {
                    r#type: Type::File,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 65521,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::Section,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 1,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::Section,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 2,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::Section,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 3,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "$d".into(),
                info: Info {
                    r#type: Type::NoType,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 2,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::Section,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 5,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "".into(),
                info: Info {
                    r#type: Type::Section,
                    binding: Binding::Local,
                },
                other: Visibility::Default,
                shndx: 4,
                value: 0,
                size: 0,
            },
            Symbol {
                name: "x".into(),
                info: Info {
                    r#type: Type::Object,
                    binding: Binding::Global,
                },
                other: Visibility::Default,
                shndx: 2,
                value: 0,
                size: 4,
            },
        ];

        assert_eq!(symbols, want);
    }
}
