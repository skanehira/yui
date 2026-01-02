use std::borrow::Cow;
use std::collections::HashMap;

use crate::elf::section::{self, SectionType};
use crate::elf::symbol;
use crate::elf::ELF;
use crate::error::Result;

use super::output::{ResolvedSymbol, Section};
use super::Linker;

/// Base address for the executable
pub static BASE_ADDR: u64 = 0x400000;

impl Linker {
    pub fn layout_sections(
        &self,
        resolved_symbols: &mut HashMap<String, ResolvedSymbol>,
    ) -> Result<(Vec<Section<'static>>, HashMap<String, usize>)> {
        let output_sections = self.merge_sections(&self.objects, resolved_symbols, BASE_ADDR)?;

        let latest_section_offset = output_sections.iter().last().unwrap().offset;
        let (symtab_section, strtab_section) =
            self.make_symbol_section(latest_section_offset, resolved_symbols);

        let mut shstrtab: Vec<u8> = Vec::new();
        let mut section_name_offsets: HashMap<String, usize> = HashMap::new();

        // add null string
        shstrtab.push(0);

        for section in output_sections.iter() {
            section_name_offsets.insert(section.name.to_string(), shstrtab.len());
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

        let shstrtab_section = Section {
            name: Cow::Borrowed(".shstrtab"),
            r#type: SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: align(symtab_section.offset + symtab_section.size, 8),
            size: shstrtab.len() as u64,
            data: Cow::Owned(shstrtab),
            align: 1,
        };

        // Combine sections using push instead of concat to avoid clone
        let mut section_tables = output_sections;
        section_tables.push(symtab_section);
        section_tables.push(strtab_section);
        section_tables.push(shstrtab_section);

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
        resolved_symbols: &mut HashMap<String, ResolvedSymbol>,
        base_addr: u64,
    ) -> Result<Vec<Section<'static>>> {
        let mut raw_text_section = vec![];
        let mut raw_data_section = vec![];

        let mut text_offsets = HashMap::new();
        let mut data_offsets = HashMap::new();

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

        let text_section = Section {
            name: Cow::Borrowed(".text"),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::ExecInstr],
            addr: text_addr,
            offset: text_offset,
            size: raw_text_section.len() as u64,
            data: Cow::Owned(raw_text_section),
            align: 4,
        };

        let data_offset = text_offset + text_section.size;
        let data_base_addr = align(text_section.addr + text_section.size + 0x10000, 4);
        let data_section = Section {
            name: Cow::Borrowed(".data"),
            r#type: section::SectionType::ProgBits,
            flags: vec![section::SectionFlag::Alloc, section::SectionFlag::Write],
            addr: data_base_addr,
            offset: data_offset,
            size: raw_data_section.len() as u64,
            data: Cow::Owned(raw_data_section),
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

    pub(super) fn make_symbol_section(
        &self,
        latest_section_offset: u64,
        resolved_symbols: &HashMap<String, ResolvedSymbol>,
    ) -> (Section<'static>, Section<'static>) {
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
        write_symbol_entry(&mut symtab, 0, 0, 0, 0, 0, 0);

        for symbol in symbols.iter() {
            write_symbol_entry(
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

        let strtab_section = Section {
            name: Cow::Borrowed(".strtab"),
            r#type: SectionType::StrTab,
            flags: vec![],
            addr: 0,
            offset: latest_section_offset + 1,
            size: strtab.len() as u64,
            data: Cow::Owned(strtab),
            align: 1,
        };

        let symtab_section = Section {
            name: Cow::Borrowed(".symtab"),
            r#type: SectionType::SymTab,
            flags: vec![],
            addr: 0,
            offset: align(strtab_section.offset + strtab_section.size, 8),
            size: symtab.len() as u64,
            data: Cow::Owned(symtab),
            align: 8,
        };

        (symtab_section, strtab_section)
    }
}

fn write_symbol_entry(
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
