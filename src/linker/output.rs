use crate::elf::{section, symbol};

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub r#type: section::SectionType,
    pub flags: Vec<section::SectionFlag>,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub data: Vec<u8>,
    pub align: u64,
}

#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub info: symbol::Info,
    pub shndx: u16,
    pub object_index: usize,
    pub is_defined: bool,
}

impl ResolvedSymbol {
    pub fn is_stronger_than(&self, other: &Self) -> bool {
        match (self.info.binding, other.info.binding) {
            (symbol::Binding::Local, _) => true,
            (_, symbol::Binding::Weak) => true,
            (symbol::Binding::Global, symbol::Binding::Global) => false,
            (symbol::Binding::Global, symbol::Binding::Local) => false,
            (symbol::Binding::Weak, _) => false,
            _ => false,
        }
    }
}
