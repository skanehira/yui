#![allow(clippy::too_many_arguments)]

pub mod output;

use std::collections::HashMap;
use std::fs;
use std::io::{self, SeekFrom};
use std::path::Path;

use crate::elf::ELF;
use crate::elf::symbol::{self, SymbolIndex};
use crate::elf::{header, program_header, relocation, section, segument};
use crate::parser;

static BASE_ADDR: u64 = 0x400000;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Default)]
pub struct Linker {
    objects: Vec<ELF>,
}

impl Linker {
    pub fn new() -> Self {
        Linker {
            objects: Vec::new(),
        }
    }

    pub fn add_objects(&mut self, paths: &[&Path]) -> Result<(), Error> {
        for path in paths {
            let obj = fs::read(path)?;
            let elf = parser::parse_elf(&obj)?.1;
            self.objects.push(elf);
        }
        Ok(())
    }

    pub fn link_to_file(&mut self, inputs: Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
        for input in inputs {
            let obj = parser::parse_elf(&input)?.1;
            self.objects.push(obj);
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

    fn make_symbol_section(
        &self,
        latest_section_offset: u64,
        resolved_symbols: &HashMap<String, output::ResolvedSymbol>,
    ) -> (output::Section, output::Section) {
        // symbol string table
        // includes null string
        let mut strtab: Vec<u8> = vec![0];
        // symbol table
        let mut symtab: Vec<u8> = Vec::new();

        let mut symbols: Vec<_> = resolved_symbols.values().collect();

        // Sort symbols properly: NULL > Local > Global > Weak
        symbols.sort_by(|&a, &b| {
            match (a.info.binding, b.info.binding) {
                // Local symbols always come first
                (symbol::Binding::Local, symbol::Binding::Local) => std::cmp::Ordering::Equal,
                (symbol::Binding::Local, _) => std::cmp::Ordering::Less,
                (_, symbol::Binding::Local) => std::cmp::Ordering::Greater,

                // Global symbols come before weak symbols
                (symbol::Binding::Global, symbol::Binding::Global) => std::cmp::Ordering::Equal,
                (symbol::Binding::Global, _) => std::cmp::Ordering::Less,
                (_, symbol::Binding::Global) => std::cmp::Ordering::Greater,

                // Otherwise equivalent (both weak, etc.)
                _ => std::cmp::Ordering::Equal,
            }
        });

        // add null symbol
        self.write_symbol_entry(&mut symtab, 0, 0, 0, 0, 0, 0);

        for symbol in symbols.iter() {
            self.write_symbol_entry(
                &mut symtab,
                strtab.len() as u32,
                symbol.value,
                symbol.size,
                symbol.info.into(),
                0, // st_other
                symbol.shndx,
            );

            strtab.extend_from_slice(symbol.name.as_bytes());
            strtab.push(0);
        }

        let strtab_section = output::Section {
            name: ".strtab".to_string(),
            r#type: section::SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: latest_section_offset + 1,
            size: strtab.len() as u64,
            data: strtab,
            align: 1,
        };

        let symtab_section = output::Section {
            name: ".symtab".to_string(),
            r#type: section::SectionType::SymTab,
            flags: vec![],
            addr: 0,
            offset: align(strtab_section.offset + strtab_section.size, 8),
            size: symtab.len() as u64,
            data: symtab,
            align: 8,
        };

        (symtab_section, strtab_section)
    }

    fn write_executable<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        resolved_symbols: HashMap<String, output::ResolvedSymbol>,
        section_tables: Vec<output::Section>,
        section_name_offsets: HashMap<String, usize>,
    ) -> Result<(), Error> {
        let Some(output::ResolvedSymbol { value: entry, .. }) = resolved_symbols.get("_start")
        else {
            return Err("sybmol _start not found".into());
        };

        let elf_header = self.create_elf_header(*entry, &section_tables);

        writer.write_all(&elf_header.to_vec())?;

        let program_headers = self.create_program_headers(&section_tables);
        self.write_program_headers(writer, &program_headers)?;

        self.write_sections(writer, &section_tables)?;

        writer.seek(SeekFrom::Start(elf_header.shoff))?;

        // null section header
        self.write_section_header(writer, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)?;

        // section headers
        for section in section_tables.iter() {
            let name_offset = *section_name_offsets.get(&section.name).unwrap_or(&0);
            let sh_type = section.r#type as u32;
            let mut sh_flags: u64 = 0;
            for f in &section.flags {
                sh_flags |= *f as u64;
            }

            let (sh_link, sh_info) = if section.name == ".symtab" {
                let strtab_idx = section_tables
                    .iter()
                    .position(|s| s.name == ".strtab")
                    .map(|i| i + 1) // includes null section
                    .unwrap_or(0) as u32;

                let local_sym_count = resolved_symbols
                    .values()
                    .filter(|s| s.info.binding == crate::elf::symbol::Binding::Local)
                    .count() as u32
                    + 1; // include null symbol

                (strtab_idx, local_sym_count)
            } else {
                (0, 0)
            };

            let sh_entsize = if section.name == ".symtab" { 24 } else { 0 };

            self.write_section_header(
                writer,
                name_offset as u32,
                sh_type,
                sh_flags,
                section.addr,
                section.offset,
                section.size,
                sh_link,
                sh_info,
                section.align,
                sh_entsize,
            )?;
        }

        Ok(())
    }

    fn create_elf_header(&self, entry: u64, section_tables: &[output::Section]) -> header::Header {
        // section header offset is after all sections
        let shoff = section_tables
            .iter()
            .map(|s| s.offset + align(s.size, 8))
            .max()
            .unwrap();

        // the number of entries in the section includes null section header
        let shnum = (section_tables.len() + 1) as u16;

        let shstrndx = section_tables
            .iter()
            .position(|s| s.name == ".shstrtab")
            .map(|i| (i + 1))
            .unwrap_or(0) as u16;

        header::Header {
            ident: header::Ident {
                class: header::Class::Bit64,
                data: header::Data::Lsb,
                version: header::IdentVersion::Current,
                os_abi: header::OSABI::SystemV,
                abi_version: 0,
            },
            r#type: header::Type::Exec,
            machine: header::Machine::AArch64,
            version: header::Version::Current,
            entry,
            phoff: 64,
            shoff,
            flags: 0,
            ehsize: 64,
            phentsize: 56,
            phnum: self.count_program_headers(section_tables),
            shentsize: 64, // the entry size for the section header table is 64 bytes
            shnum,
            shstrndx,
        }
    }

    fn count_program_headers(&self, output_sections: &[output::Section]) -> u16 {
        let sym_map = HashMap::<&str, ()>::from_iter([
            (".text", ()),
            (".data", ()),
            (".bss", ()),
            (".rodata", ()),
            (".got", ()),
            (".plt", ()),
            (".dynamic", ()),
            (".interp", ()),
            (".note", ()),
        ]);
        output_sections
            .iter()
            .filter(|s| sym_map.contains_key(s.name.as_str()))
            .count() as u16
    }

    fn create_program_headers(
        &self,
        output_sections: &[output::Section],
    ) -> Vec<program_header::ProgramHeader> {
        let mut program_headers = Vec::new();

        for s in output_sections.iter() {
            match s.name.as_str() {
                ".text" => {
                    let text_ph = program_header::ProgramHeader {
                        r#type: segument::Type::Load,
                        flags: vec![segument::Flag::Readable, segument::Flag::Executable],
                        offset: 0,
                        vaddr: BASE_ADDR,
                        paddr: BASE_ADDR,
                        filesz: s.size,
                        memsz: s.size,
                        align: 0x10000,
                    };
                    program_headers.push(text_ph);
                }
                ".data" => {
                    let data_ph = program_header::ProgramHeader {
                        r#type: segument::Type::Load,
                        flags: vec![segument::Flag::Readable, segument::Flag::Writable],
                        offset: s.offset,
                        vaddr: s.addr,
                        paddr: s.addr,
                        filesz: s.size,
                        memsz: s.size,
                        align: 0x10000,
                    };
                    program_headers.push(data_ph);
                }
                _ => {
                    // TODO
                }
            }
        }

        program_headers
    }

    fn write_program_headers<W: std::io::Write>(
        &self,
        writer: &mut W,
        headers: &[program_header::ProgramHeader],
    ) -> io::Result<()> {
        for ph in headers {
            let mut flag: u32 = 0;
            for f in &ph.flags {
                flag |= *f as u32;
            }

            let bytes = &[
                (ph.r#type as u32).to_le_bytes().as_slice(),
                flag.to_le_bytes().as_slice(),
                ph.offset.to_le_bytes().as_slice(),
                ph.vaddr.to_le_bytes().as_slice(),
                ph.paddr.to_le_bytes().as_slice(),
                ph.filesz.to_le_bytes().as_slice(),
                ph.memsz.to_le_bytes().as_slice(),
                ph.align.to_le_bytes().as_slice(),
            ]
            .concat();

            writer.write_all(bytes)?
        }

        Ok(())
    }

    fn write_sections<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        sections: &[output::Section],
    ) -> io::Result<()> {
        for section in sections {
            writer.seek(SeekFrom::Start(section.offset))?;
            writer.write_all(&section.data)?;
        }

        Ok(())
    }

    pub fn resolve_symbols(&self) -> Result<HashMap<String, output::ResolvedSymbol>, Error> {
        let mut resolved_symbols: HashMap<String, output::ResolvedSymbol> = HashMap::new();
        let mut duplicate_symbols = vec![];

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for symbol in &obj.symbols {
                let new_symbol = output::ResolvedSymbol {
                    name: symbol.name.clone(),
                    value: symbol.value,
                    size: symbol.size,
                    info: symbol.info,
                    shndx: symbol.shndx,
                    object_index: obj_idx,
                    is_defined: SymbolIndex::Undefined != symbol.shndx,
                };

                if let Some(existing) = resolved_symbols.get(&symbol.name) {
                    if new_symbol.is_defined && existing.is_defined {
                        if new_symbol.is_stronger_than(existing) {
                            resolved_symbols.insert(symbol.name.clone(), new_symbol);
                        } else {
                            duplicate_symbols.push(symbol.name.clone());
                        }
                    } else if new_symbol.is_defined && !existing.is_defined {
                        resolved_symbols.insert(symbol.name.clone(), new_symbol);
                    }
                } else {
                    resolved_symbols.insert(symbol.name.clone(), new_symbol);
                }
            }
        }

        if !duplicate_symbols.is_empty() {
            return Err(format!("Duplicate symbols: {:?}", duplicate_symbols).into());
        }

        let unresolved_symbols: Vec<_> = resolved_symbols
            .iter()
            .filter_map(|(_, symbol)| {
                if symbol.is_defined {
                    return None;
                }
                Some(symbol.name.clone())
            })
            .collect();

        if !unresolved_symbols.is_empty() {
            return Err(format!("There are unresolved symbols: {:?}", unresolved_symbols).into());
        }

        Ok(resolved_symbols)
    }

    pub fn layout_sections(
        &self,
        resolved_symbols: &mut HashMap<String, output::ResolvedSymbol>,
    ) -> Result<(Vec<output::Section>, HashMap<String, usize>), Error> {
        let output_sections = self.merge_sections(&self.objects, resolved_symbols, BASE_ADDR)?;

        let latest_section_offset = output_sections.iter().last().unwrap().offset;
        let (symtab_section, strtab_section) =
            self.make_symbol_section(latest_section_offset, resolved_symbols);

        let mut shstrtab: Vec<u8> = Vec::new();
        let mut section_name_offsets: HashMap<String, usize> = HashMap::new();

        // add null string
        shstrtab.push(0);

        for section in output_sections.iter() {
            section_name_offsets.insert(section.name.clone(), shstrtab.len());
            shstrtab.extend_from_slice(section.name.as_bytes());
            shstrtab.push(0);
        }

        section_name_offsets.insert(".strtab".to_string(), shstrtab.len());
        shstrtab.extend_from_slice(".strtab".as_bytes());
        shstrtab.push(0);

        section_name_offsets.insert(".symtab".to_string(), shstrtab.len());
        shstrtab.extend_from_slice(".symtab".as_bytes());
        shstrtab.push(0);

        section_name_offsets.insert(".shstrtab".to_string(), shstrtab.len());
        shstrtab.extend_from_slice(".shstrtab".as_bytes());
        shstrtab.push(0);

        let shstrtab_section = output::Section {
            name: ".shstrtab".to_string(),
            r#type: section::SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: align(symtab_section.offset + symtab_section.size, 8),
            size: shstrtab.len() as u64,
            data: shstrtab,
            align: 1,
        };

        let section_tables = [
            // .text, .data, .bss, etc...
            output_sections.as_slice(),
            &[
                symtab_section.clone(),
                strtab_section.clone(),
                shstrtab_section.clone(),
            ],
        ]
        .concat();

        Ok((section_tables, section_name_offsets))
    }

    /// Merges sections from multiple ELF object files into the output executable.
    ///
    /// This method:
    /// 1. Combines all `.text` sections from input objects into one output `.text` section
    /// 2. Combines all `.data` sections from input objects into one output `.data` section
    /// 3. Updates symbol addresses based on their new positions in the merged sections
    /// 4. Applies relocations to the merged sections
    ///
    /// # Arguments
    ///
    /// * `objects` - A slice of ELF object files to be linked
    /// * `resolved_symbols` - A mutable reference to a HashMap mapping symbol names to their resolved locations
    /// * `base_addr` - The base address where the text section should be loaded
    ///
    /// # Returns
    ///
    /// * `Result<Vec<output::Section>, Error>` - A vector of output sections or an error
    fn merge_sections(
        &self,
        objects: &[ELF],
        resolved_symbols: &mut HashMap<String, output::ResolvedSymbol>,
        base_addr: u64,
    ) -> Result<Vec<output::Section>, Error> {
        let mut raw_text_section = vec![];
        let mut raw_data_section = vec![];

        let mut text_offsets: HashMap<(usize, u16), usize> = HashMap::new();
        let mut data_offsets: HashMap<(usize, u16), usize> = HashMap::new();

        let mut text_current_offset = 0;
        let mut data_current_offset = 0;

        for (obj_idx, obj) in objects.iter().enumerate() {
            for (section_idx, section) in obj.section_headers.iter().enumerate() {
                match section.name.as_str() {
                    ".text" => {
                        text_offsets.insert((obj_idx, section_idx as u16), text_current_offset);
                        raw_text_section.extend_from_slice(&section.section_raw_data);
                        text_current_offset += section.section_raw_data.len();
                    }
                    ".data" => {
                        data_offsets.insert((obj_idx, section_idx as u16), data_current_offset);
                        raw_data_section.extend_from_slice(&section.section_raw_data);
                        data_current_offset += section.section_raw_data.len();
                    }
                    _ => {
                        // TODO
                    }
                }
            }
        }

        // Place after ELF header and program header
        let text_offset = 0x100;
        let text_addr = align(base_addr + text_offset, 4);

        let text_section = output::Section {
            name: ".text".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::ExecInstr],
            addr: text_addr,
            offset: text_offset,
            size: raw_text_section.len() as u64,
            data: raw_text_section,
            align: 4,
        };

        let data_offset = text_offset + text_section.size;
        let data_base_addr = align(text_section.addr + text_section.size + 0x10000, 4);
        let data_section = output::Section {
            name: ".data".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::Write],
            addr: data_base_addr,
            offset: data_offset,
            size: raw_data_section.len() as u64,
            data: raw_data_section,
            align: 4,
        };

        for symbol in resolved_symbols.values_mut() {
            if let Some(&offset) = text_offsets.get(&(symbol.object_index, symbol.shndx)) {
                // offset: section offset after merged
                // symbol.value: offset in the section
                symbol.value = text_section.addr + (offset + symbol.value as usize) as u64;
            } else if let Some(&offset) = data_offsets.get(&(symbol.object_index, symbol.shndx)) {
                symbol.value = data_section.addr + (offset + symbol.value as usize) as u64;
            }
        }

        let mut output_sections = vec![text_section, data_section];

        self.apply_relocations(&mut output_sections, resolved_symbols)?;

        Ok(output_sections)
    }

    pub fn apply_relocations(
        &self,
        output_sections: &mut [output::Section],
        resolved_symbols: &HashMap<String, output::ResolvedSymbol>,
    ) -> Result<(), Error> {
        let section_indices: HashMap<String, usize> = output_sections
            .iter()
            .enumerate()
            .map(|(i, sec)| (sec.name.clone(), i))
            .collect();

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for reloc in &obj.relocations {
                self.process_relocation(
                    obj_idx,
                    reloc,
                    output_sections,
                    &section_indices,
                    resolved_symbols,
                )?;
            }
        }

        Ok(())
    }

    fn process_relocation(
        &self,
        obj_idx: usize,
        reloc: &relocation::RelocationAddend,
        output_sections: &mut [output::Section],
        section_indices: &HashMap<String, usize>,
        resolved_symbols: &HashMap<String, output::ResolvedSymbol>,
    ) -> Result<(), String> {
        match reloc.info.r#type {
            relocation::RelocationType::Aarch64AdrPrelLo21 => {
                let symbol_index = reloc.info.symbol_index as usize;
                if symbol_index >= self.objects[obj_idx].symbols.len() {
                    return Err(format!("Symbol index out of range: {}", symbol_index));
                }

                let symbol_name = &self.objects[obj_idx].symbols[symbol_index].name;

                let resolved_symbol = resolved_symbols
                    .get(symbol_name)
                    .ok_or_else(|| format!("Symbol is not resolved: {}", symbol_name))?;

                if !resolved_symbol.is_defined {
                    return Err(format!("Relocation to undefined symbol: {}", symbol_name));
                }

                let text_section_idx = section_indices
                    .get(".text")
                    .ok_or_else(|| "Relocation target section not found: .text".to_string())?;

                let target_section = &mut output_sections[*text_section_idx];

                if reloc.offset as usize >= target_section.data.len() {
                    return Err(format!("Relocation offset out of range: {}", reloc.offset));
                }

                let instruction_addr = target_section.addr + reloc.offset;
                let symbol_addr = resolved_symbol.value;

                // Calculates relative address with symbol (target address - PC value)
                // PC is the current instruction address (instruction_addr)
                let relative_addr =
                    ((symbol_addr as i64) - (instruction_addr as i64) + reloc.addend) as i32;

                let pos = reloc.offset as usize;
                let instruction =
                    u32::from_le_bytes(target_section.data[pos..pos + 4].try_into().unwrap());

                // Keeps opcode and register portion of the ADR instruction
                // ADR instruction format: 0bxxx10000 iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii
                // x: opcode, i: immbit, d: destination register
                let opcode_rd = instruction & 0x9F00001F; // オペコードとレジスタ部分を保持

                // Encoding of ADR instructions (based on the ARMv8 Architecture Reference Manual)
                // immhi: upper 19 bits of immediate (bits 5-23)
                // immlo: lower 2 bits of immediate (bits 29-30)
                let immlo = ((relative_addr & 0x3) as u32) << 29;
                let immhi = (((relative_addr >> 2) & 0x7FFFF) as u32) << 5;

                let new_instruction = opcode_rd | immlo | immhi;

                target_section.data[pos..pos + 4]
                    .copy_from_slice(new_instruction.to_le_bytes().as_slice());
            }
            _ => {
                // TODO
            }
        }

        Ok(())
    }

    fn write_symbol_entry(
        &self,
        data: &mut Vec<u8>,
        st_name: u32,
        st_value: u64,
        st_size: u64,
        st_info: u8,
        st_other: u8,
        st_shndx: u16,
    ) {
        data.extend_from_slice(&st_name.to_le_bytes());
        data.push(st_info);
        data.push(st_other);
        data.extend_from_slice(&st_shndx.to_le_bytes());
        data.extend_from_slice(&st_value.to_le_bytes());
        data.extend_from_slice(&st_size.to_le_bytes());
    }

    fn write_section_header<W: std::io::Write>(
        &self,
        writer: &mut W,
        name: u32,
        sh_type: u32,
        sh_flags: u64,
        sh_addr: u64,
        sh_offset: u64,
        sh_size: u64,
        sh_link: u32,
        sh_info: u32,
        sh_addralign: u64,
        sh_entsize: u64,
    ) -> io::Result<()> {
        let bytes = [
            name.to_le_bytes().as_slice(),
            sh_type.to_le_bytes().as_slice(),
            sh_flags.to_le_bytes().as_slice(),
            sh_addr.to_le_bytes().as_slice(),
            sh_offset.to_le_bytes().as_slice(),
            sh_size.to_le_bytes().as_slice(),
            sh_link.to_le_bytes().as_slice(),
            sh_info.to_le_bytes().as_slice(),
            sh_addralign.to_le_bytes().as_slice(),
            sh_entsize.to_le_bytes().as_slice(),
        ]
        .concat();

        writer.write_all(&bytes)
    }
}

/// Aligns a value to the specified power-of-2 alignment boundary.
///
/// This function rounds up `value` to the next multiple of `alignment`.
///
/// # Arguments
///
/// * `value` - The value to align
/// * `alignment` - The alignment boundary (must be a power of 2)
///
/// # Returns
///
/// The smallest value greater than or equal to `value` that is a multiple of `alignment`
///
/// # Example
///
/// ```
/// use yui::linker::align;
/// let aligned = align(10, 8); // Results in 16
/// let already_aligned = align(16, 8); // Remains 16
/// ```
pub fn align(value: u64, alignment: u64) -> u64 {
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
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
