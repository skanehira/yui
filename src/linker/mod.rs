pub mod output;
mod relocation;
mod section;
mod symbol;
mod writer;

use std::fs;
use std::path::Path;

use crate::elf::ELF;
use crate::error::{LinkerError, Result};
use crate::parser;

pub use section::{BASE_ADDR, align};

#[derive(Debug, Default)]
pub struct Linker {
    objects: Vec<ELF>,
    object_names: Vec<String>,
}

impl Linker {
    pub fn new() -> Self {
        Linker {
            objects: Vec::new(),
            object_names: Vec::new(),
        }
    }

    pub fn add_objects(&mut self, paths: &[&Path]) -> Result<()> {
        for path in paths {
            let obj = fs::read(path).map_err(|e| LinkerError::Io {
                error: e,
                context: Some(format!("reading file: {}", path.display())),
            })?;
            let elf = parser::parse_elf(&obj)
                .map_err(|e| match e {
                    nom::Err::Error(parse_err) | nom::Err::Failure(parse_err) => {
                        LinkerError::Parse {
                            error: parse_err,
                            context: Some(format!("parsing file: {}", path.display())),
                        }
                    }
                    nom::Err::Incomplete(_) => LinkerError::Generic {
                        message: "Incomplete input data".to_string(),
                        context: Some(format!("parsing file: {}", path.display())),
                    },
                })?
                .1;
            self.objects.push(elf);
            self.object_names.push(path.display().to_string());
        }
        Ok(())
    }

    pub fn link_to_file(&mut self, inputs: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        for (idx, input) in inputs.iter().enumerate() {
            let obj = parser::parse_elf(input)
                .map_err(|e| match e {
                    nom::Err::Error(parse_err) | nom::Err::Failure(parse_err) => {
                        LinkerError::Parse {
                            error: parse_err,
                            context: Some(format!("parsing input object {}", idx)),
                        }
                    }
                    nom::Err::Incomplete(_) => LinkerError::Generic {
                        message: "Incomplete input data".to_string(),
                        context: Some(format!("parsing input object {}", idx)),
                    },
                })?
                .1;
            self.objects.push(obj);
            self.object_names.push(format!("input_{}", idx));
        }
        let mut resolved_symbols = self.resolve_symbols()?;
        let (output_sections, section_name_offsets) =
            self.layout_sections(&mut resolved_symbols)?;
        let mut out = std::io::Cursor::new(Vec::new());

        self.write_executable(
            &mut out,
            resolved_symbols,
            output_sections,
            section_name_offsets,
        )?;

        Ok(out.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::section;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn test_symbol_resolution() {
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let mut linker = Linker::new();
        linker.add_objects(&[main_o, sub_o]).unwrap();

        let resolved_symbols = linker.resolve_symbols().unwrap();

        assert!(
            resolved_symbols.contains_key("_start"),
            "Symbol _start is not resolved"
        );
        assert!(
            resolved_symbols.contains_key("x"),
            "Symbol x is not resolved"
        );

        let x_symbol = resolved_symbols.get("x").unwrap();
        assert_eq!(
            x_symbol.object_index, 1,
            "Invalid object source for symbol x"
        );
    }

    #[test]
    fn test_section_layout() {
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let mut linker = Linker::new();
        linker.add_objects(&[main_o, sub_o]).unwrap();

        let mut resolved_symbols = linker.resolve_symbols().unwrap();

        let output_sections = linker.layout_sections(&mut resolved_symbols).unwrap().0;

        let mut section_iter = output_sections.iter();
        assert!(
            section_iter.any(|s| s.name == ".text"),
            "No .text section found"
        );
        assert!(
            section_iter.any(|s| s.name == ".data"),
            "No .data section found"
        );

        let text_idx = output_sections
            .iter()
            .position(|s| s.name == ".text")
            .unwrap();
        let data_idx = output_sections
            .iter()
            .position(|s| s.name == ".data")
            .unwrap();
        assert!(
            text_idx < data_idx,
            "Invalid section order: .text should be before .data"
        );

        let text_section = &output_sections[text_idx];
        assert!(
            text_section
                .flags
                .contains(&section::SectionFlag::ExecInstr),
            "No executable flag in .text section"
        );

        let data_section = &output_sections[data_idx];
        assert!(
            data_section.flags.contains(&section::SectionFlag::Write),
            "No writable flag in .data section"
        );

        assert!(text_section.addr > 0, "Invalid address for .text section");
        assert!(
            data_section.addr > text_section.addr,
            ".data section should be placed after .text section"
        );

        assert_eq!(
            text_section.addr, 0x400100,
            "Start address of .text section differs from expected value"
        );

        assert_eq!(
            data_section.addr, 0x410110,
            "Start address of .data section differs from expected value"
        );

        assert!(text_section.size > 0, "Invalid size for .text section");
        assert!(data_section.size > 0, "Invalid size for .data section");

        let x_symbol = resolved_symbols.get("x").unwrap();
        assert!(
            x_symbol.value >= data_section.addr
                && x_symbol.value < data_section.addr + data_section.size,
            "Address of symbol x is not within .data section"
        );

        let start_symbol = resolved_symbols.get("_start").unwrap();
        assert!(
            start_symbol.value >= text_section.addr
                && start_symbol.value < text_section.addr + text_section.size,
            "Address of symbol _start is not within .text section"
        );

        let expected_start_addr = text_section.addr;
        assert_eq!(
            start_symbol.value, expected_start_addr,
            "Address of symbol _start differs from expected value"
        );

        let expected_x_addr = data_section.addr;
        assert_eq!(
            x_symbol.value, expected_x_addr,
            "Address of symbol x differs from expected value"
        );
    }

    #[test]
    fn test_apply_relocations() {
        let main_o = Path::new("src/parser/fixtures/main.o");
        let sub_o = Path::new("src/parser/fixtures/sub.o");

        let mut linker = Linker::new();
        linker.add_objects(&[main_o, sub_o]).unwrap();

        let mut resolved_symbols = linker.resolve_symbols().unwrap();
        let mut output_sections = linker.layout_sections(&mut resolved_symbols).unwrap().0;
        linker
            .apply_relocations(&mut output_sections, &resolved_symbols)
            .unwrap();

        let text_section = output_sections.iter().find(|s| s.name == ".text").unwrap();

        let offset = 0;
        assert!(
            offset + 4 <= text_section.data.len(),
            "Instruction offset is out of range"
        );

        let instruction =
            u32::from_le_bytes(text_section.data[offset..offset + 4].try_into().unwrap());

        let addr_field = instruction & 0x1FFFFF;

        assert!(addr_field != 0, "Address field not updated by relocation");
    }
}
