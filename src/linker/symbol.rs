use std::collections::HashMap;

use crate::elf::symbol::SymbolIndex;
use crate::error::{LinkerError, ObjectContext, Result, UnresolvedSymbol};

use super::Linker;
use super::output::ResolvedSymbol;

impl Linker {
    pub fn resolve_symbols(&self) -> Result<HashMap<String, ResolvedSymbol>> {
        let mut resolved_symbols: HashMap<String, ResolvedSymbol> = HashMap::new();
        let mut duplicate_symbols = HashMap::new();

        for (obj_idx, obj) in self.objects.iter().enumerate() {
            let file_name = self
                .object_names
                .get(obj_idx)
                .cloned()
                .unwrap_or_else(|| format!("object_{}", obj_idx));

            for symbol in &obj.symbols {
                let new_symbol = ResolvedSymbol {
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
                            let existing_file_name = self
                                .object_names
                                .get(existing.object_index)
                                .cloned()
                                .unwrap_or_else(|| format!("object_{}", existing.object_index));
                            duplicate_symbols.insert(
                                symbol.name.clone(),
                                (
                                    ObjectContext {
                                        file_name: existing_file_name,
                                        object_index: existing.object_index,
                                    },
                                    ObjectContext {
                                        file_name: file_name.clone(),
                                        object_index: obj_idx,
                                    },
                                ),
                            );
                        }
                    } else if new_symbol.is_defined && !existing.is_defined {
                        resolved_symbols.insert(symbol.name.clone(), new_symbol);
                    }
                } else {
                    resolved_symbols.insert(symbol.name.clone(), new_symbol);
                }
            }
        }

        if let Some((symbol_name, (first_def, dup_def))) = duplicate_symbols.into_iter().next() {
            return Err(LinkerError::duplicate_symbol(
                symbol_name,
                first_def,
                dup_def,
            ));
        }

        let unresolved_symbols: Vec<UnresolvedSymbol> = resolved_symbols
            .iter()
            .filter_map(|(_, symbol)| {
                if symbol.is_defined {
                    return None;
                }
                Some(UnresolvedSymbol {
                    name: symbol.name.clone(),
                    referenced_from: vec![ObjectContext {
                        file_name: self
                            .object_names
                            .get(symbol.object_index)
                            .cloned()
                            .unwrap_or_else(|| format!("object_{}", symbol.object_index)),
                        object_index: symbol.object_index,
                    }],
                })
            })
            .collect();

        if !unresolved_symbols.is_empty() {
            return Err(LinkerError::unresolved_symbols(unresolved_symbols));
        }

        Ok(resolved_symbols)
    }
}
