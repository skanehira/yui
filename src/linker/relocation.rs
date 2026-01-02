use std::collections::HashMap;

use crate::elf::relocation;
use crate::error::{LinkerError, ObjectContext, Result};

use super::Linker;
use super::output::{ResolvedSymbol, Section};

impl Linker {
    pub fn apply_relocations(
        &self,
        output_sections: &mut [Section<'static>],
        resolved_symbols: &HashMap<String, ResolvedSymbol>,
    ) -> Result<()> {
        let section_indices: HashMap<String, usize> = output_sections
            .iter()
            .enumerate()
            .map(|(i, sec)| (sec.name.to_string(), i))
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
        output_sections: &mut [Section<'static>],
        section_indices: &HashMap<String, usize>,
        resolved_symbols: &HashMap<String, ResolvedSymbol>,
    ) -> Result<()> {
        let object_file_name = self
            .object_names
            .get(obj_idx)
            .cloned()
            .unwrap_or_else(|| format!("object_{}", obj_idx));
        match reloc.info.r#type {
            relocation::RelocationType::Aarch64AdrPrelLo21 => {
                let symbol_index = reloc.info.symbol_index as usize;
                if symbol_index >= self.objects[obj_idx].symbols.len() {
                    return Err(LinkerError::relocation_error(
                        format!("Symbol index out of range: {}", symbol_index),
                        None,
                        Some(ObjectContext {
                            file_name: object_file_name.clone(),
                            object_index: obj_idx,
                        }),
                        Some("R_AARCH64_ADR_PREL_LO21".to_string()),
                    ));
                }

                let symbol_name = &self.objects[obj_idx].symbols[symbol_index].name;

                let resolved_symbol = resolved_symbols.get(symbol_name).ok_or_else(|| {
                    LinkerError::relocation_error(
                        "Symbol is not resolved",
                        Some(symbol_name.clone()),
                        Some(ObjectContext {
                            file_name: object_file_name.clone(),
                            object_index: obj_idx,
                        }),
                        Some("R_AARCH64_ADR_PREL_LO21".to_string()),
                    )
                })?;

                let text_section_idx =
                    section_indices
                        .get(".text")
                        .ok_or_else(|| LinkerError::SectionNotFound {
                            section_name: ".text".to_string(),
                            context: Some("applying relocation".to_string()),
                        })?;

                let target_section = &mut output_sections[*text_section_idx];

                if reloc.offset as usize >= target_section.data.len() {
                    return Err(LinkerError::relocation_error(
                        format!("Relocation offset out of range: {}", reloc.offset),
                        Some(symbol_name.clone()),
                        Some(ObjectContext {
                            file_name: object_file_name,
                            object_index: obj_idx,
                        }),
                        Some("R_AARCH64_ADR_PREL_LO21".to_string()),
                    ));
                }

                let instruction_addr = target_section.addr + reloc.offset;
                let symbol_addr = resolved_symbol.value;

                // Calculates relative address with symbol (target address - PC value)
                // PC is the current instruction address (instruction_addr)
                let relative_addr =
                    ((symbol_addr as i64) - (instruction_addr as i64) + reloc.addend) as i32;

                let pos = reloc.offset as usize;
                let data = target_section.data.to_mut();
                let instruction = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());

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

                data[pos..pos + 4].copy_from_slice(new_instruction.to_le_bytes().as_slice());
            }
        }

        Ok(())
    }
}
