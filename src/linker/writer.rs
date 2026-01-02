use std::collections::HashMap;
use std::io::{SeekFrom, Write};

use crate::elf::{header, program_header, segument};
use crate::error::{LinkerError, Result};

use super::output::{ResolvedSymbol, Section};
use super::section::{align, BASE_ADDR};
use super::Linker;

#[derive(Debug, Default)]
struct SectionHeaderEntry {
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
}

impl SectionHeaderEntry {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let bytes = [
            self.name.to_le_bytes().as_slice(),
            self.sh_type.to_le_bytes().as_slice(),
            self.sh_flags.to_le_bytes().as_slice(),
            self.sh_addr.to_le_bytes().as_slice(),
            self.sh_offset.to_le_bytes().as_slice(),
            self.sh_size.to_le_bytes().as_slice(),
            self.sh_link.to_le_bytes().as_slice(),
            self.sh_info.to_le_bytes().as_slice(),
            self.sh_addralign.to_le_bytes().as_slice(),
            self.sh_entsize.to_le_bytes().as_slice(),
        ]
        .concat();

        writer.write_all(&bytes).map_err(|e| LinkerError::Io {
            error: e,
            context: Some("writing section header".to_string()),
        })
    }
}

impl Linker {
    pub(super) fn write_executable<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        resolved_symbols: HashMap<String, ResolvedSymbol>,
        section_tables: Vec<Section<'static>>,
        section_name_offsets: HashMap<String, usize>,
    ) -> Result<()> {
        let Some(ResolvedSymbol { value: entry, .. }) = resolved_symbols.get("_start") else {
            return Err(LinkerError::MissingEntryPoint {
                entry_symbol: "_start".to_string(),
            });
        };

        let elf_header = self.create_elf_header(*entry, &section_tables);

        writer
            .write_all(&elf_header.to_vec())
            .map_err(|e| LinkerError::Io {
                error: e,
                context: Some("writing ELF header".to_string()),
            })?;

        let program_headers = self.create_program_headers(&section_tables);
        self.write_program_headers(writer, &program_headers)?;

        self.write_sections(writer, &section_tables)?;

        writer
            .seek(SeekFrom::Start(elf_header.shoff))
            .map_err(|e| LinkerError::Io {
                error: e,
                context: Some("seeking to section header table".to_string()),
            })?;

        // null section header
        SectionHeaderEntry::default().write(writer)?;

        // section headers
        for section in section_tables.iter() {
            let name_offset = *section_name_offsets
                .get(section.name.as_ref())
                .unwrap_or(&0);

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

            let entry = SectionHeaderEntry {
                name: name_offset as u32,
                sh_type: section.r#type as u32,
                sh_flags,
                sh_addr: section.addr,
                sh_offset: section.offset,
                sh_size: section.size,
                sh_link,
                sh_info,
                sh_addralign: section.align,
                sh_entsize: if section.name == ".symtab" { 24 } else { 0 },
            };
            entry.write(writer)?;
        }

        Ok(())
    }

    fn create_elf_header(&self, entry: u64, section_tables: &[Section<'static>]) -> header::Header {
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

    fn count_program_headers(&self, output_sections: &[Section<'static>]) -> u16 {
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
            .filter(|s| sym_map.contains_key(s.name.as_ref()))
            .count() as u16
    }

    fn create_program_headers(
        &self,
        output_sections: &[Section<'static>],
    ) -> Vec<program_header::ProgramHeader> {
        let mut program_headers = Vec::new();

        for s in output_sections.iter() {
            match s.name.as_ref() {
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
    ) -> Result<()> {
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

            writer.write_all(bytes).map_err(|e| LinkerError::Io {
                error: e,
                context: Some("writing program header".to_string()),
            })?
        }

        Ok(())
    }

    fn write_sections<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        sections: &[Section<'static>],
    ) -> Result<()> {
        for section in sections {
            writer
                .seek(SeekFrom::Start(section.offset))
                .map_err(|e| LinkerError::Io {
                    error: e,
                    context: Some(format!("seeking to section {} offset", section.name)),
                })?;
            writer
                .write_all(section.data.as_ref())
                .map_err(|e| LinkerError::Io {
                    error: e,
                    context: Some(format!("writing section {} data", section.name)),
                })?;
        }

        Ok(())
    }
}
