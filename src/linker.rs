#![allow(clippy::too_many_arguments)]

pub mod output;

use std::collections::HashMap;
use std::fs;
use std::io::{self, SeekFrom};
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;

use crate::elf::ELF;
use crate::elf::symbol::SymbolIndex;
use crate::elf::{header, program_header, relocation, section, segument};
use crate::parser;

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

    fn add_objects(&mut self, paths: &[&Path]) -> Result<(), Error> {
        for path in paths {
            let obj = fs::read(path)?;
            let elf = parser::parse_elf(&obj)?.1;
            self.objects.push(elf);
        }

        Ok(())
    }

    pub fn link_to_file(&mut self, output_path: &Path, input_paths: &[&Path]) -> Result<(), Error> {
        self.add_objects(input_paths)?;
        let mut resolved_symbols = self.resolve_symbols()?;
        let (output_sections, section_name_offsets) =
            self.layout_sections(&mut resolved_symbols)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .mode(0o655) // rw-r-xr-x
            .open(output_path)?;
        self.write_executable(
            &mut file,
            resolved_symbols,
            output_sections,
            section_name_offsets,
        )
    }

    fn make_symbol_section(
        &self,
        resolved_symbols: HashMap<String, output::ResolvedSymbol>,
    ) -> (output::Section, output::Section) {
        // symbol string table
        let mut strtab: Vec<u8> = Vec::new();
        // symbol table
        let mut symtab: Vec<u8> = Vec::new();

        for (name, symbol) in resolved_symbols.iter() {
            if name.is_empty() {
                continue;
            }
            strtab.extend_from_slice(name.as_bytes());
            strtab.push(0);

            let st_info = ((symbol.binding as u8) << 4) | (symbol.r#type as u8);

            self.write_symbol_entry(
                &mut symtab,
                strtab.len() as u32,
                symbol.value,
                symbol.size,
                st_info,
                0, // st_other
                symbol.shndx,
            );
        }

        let strtab_section = output::Section {
            name: ".strtab".to_string(),
            r#type: section::SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: align(0x3000, 8),
            size: strtab.len() as u64,
            data: strtab,
            align: 8,
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
        let mut elf_header = self.create_elf_header(&section_tables);

        let Some(output::ResolvedSymbol {
            value: entry_point, ..
        }) = resolved_symbols.get("_start")
        else {
            return Err("sybmol _start not found".into());
        };

        elf_header.entry = *entry_point;

        let section_data_end = section_tables
            .iter()
            .map(|s| s.offset + align(s.size, 8))
            .max()
            .unwrap_or(0x3000);

        let sh_offset = align(section_data_end, 8);
        elf_header.shoff = sh_offset;
        elf_header.shentsize = 64;
        elf_header.shnum = (section_tables.len() + 1) as u16; // include NULL section

        let shstrtab_idx = section_tables
            .iter()
            .position(|s| s.name == ".shstrtab")
            .unwrap_or(0);
        elf_header.shstrndx = (shstrtab_idx + 1) as u16;

        let program_headers = self.create_program_headers(&section_tables);

        self.write_elf_header(writer, &elf_header)?;

        self.write_program_headers(writer, &program_headers)?;

        self.write_sections(writer, &section_tables)?;

        writer.seek(SeekFrom::Start(sh_offset))?;

        self.write_section_header(writer, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)?;

        for section in section_tables.iter() {
            let name_offset = *section_name_offsets.get(&section.name).unwrap_or(&0);
            let sh_type = section.r#type as u32;
            let mut sh_flags: u64 = 0;
            for flag in &section.flags {
                let bit = match flag {
                    section::SectionFlag::Write => 0x1,
                    section::SectionFlag::Alloc => 0x2,
                    section::SectionFlag::ExecInstr => 0x4,
                    _ => 0,
                };
                sh_flags |= bit;
            }

            let (sh_link, sh_info) = if section.name == ".symtab" {
                let strtab_idx = section_tables
                    .iter()
                    .position(|s| s.name == ".strtab")
                    .unwrap_or(0)
                    + 1;
                (strtab_idx as u32, 1)
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

    fn create_elf_header(&self, output_sections: &[output::Section]) -> header::Header {
        header::Header {
            ident: header::Ident {
                class: header::Class::Bit64,
                data: header::Data::Lsb, // Little Endian
                version: header::IdentVersion::Current,
                os_abi: header::OSABI::SystemV,
                abi_version: 0,
            },
            r#type: header::Type::Exec,
            machine: header::Machine::AArch64,
            version: header::Version::Current,
            entry: 0x400000,
            phoff: 64,
            shoff: 0,
            flags: 0,
            ehsize: 64,
            phentsize: 56,
            phnum: self.count_program_headers(output_sections),
            shentsize: 0,
            shnum: 0,
            shstrndx: 0,
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
        let x: Vec<_> = output_sections
            .iter()
            .filter(|s| sym_map.contains_key(s.name.as_str()))
            .collect();
        x.len() as u16
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
                        offset: s.offset,
                        vaddr: s.addr,
                        paddr: s.addr,
                        filesz: s.size,
                        memsz: s.size,
                        align: s.align,
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
                        align: s.align,
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

    fn write_elf_header<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        header: &header::Header,
    ) -> io::Result<()> {
        let bytes = [
            &[0x7f, b'E', b'L', b'F'],
            (header.ident.class as u8).to_le_bytes().as_slice(),
            (header.ident.data as u8).to_le_bytes().as_slice(),
            (header.ident.version as u8).to_le_bytes().as_slice(),
            (header.ident.os_abi as u8).to_le_bytes().as_slice(),
            (header.ident.abi_version).to_le_bytes().as_slice(),
            [0; 7].as_slice(),
            (header.r#type as u16).to_le_bytes().as_slice(),
            (header.machine as u16).to_le_bytes().as_slice(),
            (header.version as u32).to_le_bytes().as_slice(),
            header.entry.to_le_bytes().as_slice(),
            header.phoff.to_le_bytes().as_slice(),
            header.shoff.to_le_bytes().as_slice(),
            header.flags.to_le_bytes().as_slice(),
            header.ehsize.to_le_bytes().as_slice(),
            header.phentsize.to_le_bytes().as_slice(),
            header.phnum.to_le_bytes().as_slice(),
            header.shentsize.to_le_bytes().as_slice(),
            header.shnum.to_le_bytes().as_slice(),
            header.shstrndx.to_le_bytes().as_slice(),
        ]
        .concat();

        writer.write_all(&bytes)
    }

    fn write_program_headers<W: std::io::Write>(
        &self,
        writer: &mut W,
        headers: &[program_header::ProgramHeader],
    ) -> io::Result<()> {
        for ph in headers {
            let mut flags: u32 = 0;
            for flag in &ph.flags {
                let bit = match flag {
                    segument::Flag::Executable => 0x1,
                    segument::Flag::Writable => 0x2,
                    segument::Flag::Readable => 0x4,
                    segument::Flag::MaskOS | segument::Flag::MaskProc => {
                        continue;
                    }
                };
                flags |= bit;
            }

            let bytes = &[
                (ph.r#type as u32).to_le_bytes().as_slice(),
                flags.to_le_bytes().as_slice(),
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

    pub fn has_symbol(&self, name: &str) -> bool {
        for obj in &self.objects {
            for symbol in &obj.symbols {
                if symbol.name == name {
                    return true;
                }
            }
        }
        false
    }

    pub fn has_undefined_symbol(&self, name: &str) -> bool {
        for obj in &self.objects {
            for symbol in &obj.symbols {
                if symbol.name == name && SymbolIndex::Undefined == symbol.shndx {
                    return true;
                }
            }
        }
        false
    }

    pub fn resolve_symbols(&self) -> Result<HashMap<String, output::ResolvedSymbol>, Error> {
        let mut resolved_symbols: HashMap<String, output::ResolvedSymbol> = HashMap::new();
        let mut duplicate_symbols = vec![];

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for symbol in &obj.symbols {
                if symbol.name.is_empty() {
                    continue;
                }

                let new_symbol = output::ResolvedSymbol {
                    name: symbol.name.clone(),
                    value: symbol.value,
                    size: symbol.size,
                    r#type: symbol.info.r#type,
                    binding: symbol.info.binding,
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
        let mut output_sections: Vec<output::Section> = Vec::new();

        let (text_data, text_offsets) = self.merge_sections(".text", 1);
        let (data_data, data_offsets) = self.merge_sections(".data", 2);

        let base_addr = 0x400000;
        let text_section = output::Section {
            name: ".text".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::ExecInstr],
            addr: base_addr + 0x1000,
            offset: 0x1000,
            size: text_data.len() as u64,
            data: text_data,
            align: 0x1000,
        };

        let data_addr = align(base_addr + 0x2000, 0x1000);
        let data_section = output::Section {
            name: ".data".to_string(),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::Write],
            addr: data_addr,
            offset: 0x2000,
            size: data_data.len() as u64,
            data: data_data,
            align: 0x1000,
        };

        output_sections.push(text_section);
        output_sections.push(data_section);

        self.update_symbol_addresses(
            resolved_symbols,
            &text_offsets,
            base_addr + 0x1000,
            &data_offsets,
            data_addr,
        );

        self.apply_relocations(&mut output_sections, resolved_symbols)?;

        let (symtab_section, strtab_section) = self.make_symbol_section(resolved_symbols.clone());

        let mut shstrtab: Vec<u8> = Vec::new();
        let mut section_name_offsets: HashMap<String, usize> = HashMap::new();

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

    fn merge_sections(
        &self,
        section_name: &str,
        section_index: u16,
    ) -> (Vec<u8>, HashMap<(usize, u16), usize>) {
        let mut section_data: Vec<u8> = Vec::new();
        let mut offsets: HashMap<(usize, u16), usize> = HashMap::new();
        let mut current_offset = 0;

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            for section in &obj.section_headers {
                if section.name == section_name {
                    offsets.insert((obj_idx, section_index), current_offset);
                    section_data.extend_from_slice(&section.section_raw_data);
                    current_offset += section.section_raw_data.len();
                }
            }
        }

        (section_data, offsets)
    }

    fn update_symbol_addresses(
        &self,
        resolved_symbols: &mut HashMap<String, output::ResolvedSymbol>,
        text_offsets: &HashMap<(usize, u16), usize>,
        text_base_addr: u64,
        data_offsets: &HashMap<(usize, u16), usize>,
        data_base_addr: u64,
    ) {
        for symbol in resolved_symbols.values_mut() {
            match symbol.shndx {
                1 => {
                    if let Some(&offset) = text_offsets.get(&(symbol.object_index, symbol.shndx)) {
                        symbol.value = text_base_addr + (offset + symbol.value as usize) as u64;
                    }
                }
                2 => {
                    if let Some(&offset) = data_offsets.get(&(symbol.object_index, symbol.shndx)) {
                        symbol.value = data_base_addr + (offset + symbol.value as usize) as u64;
                    }
                }
                _ => {
                    // TODO
                }
            }
        }
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
                    return Err(format!("シンボルインデックスが範囲外: {}", symbol_index));
                }

                let symbol_name = &self.objects[obj_idx].symbols[symbol_index].name;

                let resolved_symbol = resolved_symbols
                    .get(symbol_name)
                    .ok_or_else(|| format!("シンボルが解決されていません: {}", symbol_name))?;

                if !resolved_symbol.is_defined {
                    return Err(format!("未定義シンボルへの再配置: {}", symbol_name));
                }

                let text_section_idx = section_indices
                    .get(".text")
                    .ok_or_else(|| "再配置対象セクションが見つかりません: .text".to_string())?;

                let target_section = &mut output_sections[*text_section_idx];

                if reloc.offset as usize >= target_section.data.len() {
                    return Err(format!("再配置オフセットが範囲外: {}", reloc.offset));
                }

                let instruction_addr = target_section.addr + reloc.offset;
                let symbol_addr = resolved_symbol.value;

                // Calculates relative address with symbol (target address - PC value)
                // PC is the current instruction address (instruction_addr)
                let relative_addr =
                    ((symbol_addr as i64) - (instruction_addr as i64) + reloc.addend) as i32;

                let pos = reloc.offset as usize;
                let instruction = u32::from_le_bytes([
                    target_section.data[pos],
                    target_section.data[pos + 1],
                    target_section.data[pos + 2],
                    target_section.data[pos + 3],
                ]);

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

                let instruction_bytes = new_instruction.to_le_bytes();
                target_section.data[pos] = instruction_bytes[0];
                target_section.data[pos + 1] = instruction_bytes[1];
                target_section.data[pos + 2] = instruction_bytes[2];
                target_section.data[pos + 3] = instruction_bytes[3];
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

fn align(value: u64, alignment: u64) -> u64 {
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn test_basic_object_parsing() {
        let main_o = Path::new("src/parser/fixtures/main.o");

        let mut linker = Linker::new();
        linker.add_objects(&[main_o]).unwrap();

        let has_start = linker.has_symbol("_start");
        assert!(has_start, "Symbol _start not found");

        let uses_x = linker.has_undefined_symbol("x");
        assert!(uses_x, "Undefined symbol x not found");
    }

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
            text_section.addr, 0x401000,
            "Start address of .text section differs from expected value"
        );

        let expected_data_addr = align(text_section.addr + text_section.size, 0x1000);
        assert_eq!(
            data_section.addr, expected_data_addr,
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

        let instruction = u32::from_le_bytes([
            text_section.data[offset],
            text_section.data[offset + 1],
            text_section.data[offset + 2],
            text_section.data[offset + 3],
        ]);

        let addr_field = instruction & 0x1FFFFF;

        assert!(addr_field != 0, "Address field not updated by relocation");
    }
}
