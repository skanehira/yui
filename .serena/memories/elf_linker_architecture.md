# ELF Linker Architecture

## Overview
Yui is a simple ELF linker that takes multiple object files (.o) and combines them into an executable. It targets ARM64 (aarch64) Linux systems.

## Core Architecture

### 1. Parser Module (`src/parser/`)
- Uses `nom` parser combinator library for binary parsing
- Parses ELF headers, sections, symbols, and relocations
- Key functions:
  - `parse_elf()` - Main entry point for parsing ELF files
  - Individual parsers for headers, sections, symbols, relocations

### 2. ELF Data Structures (`src/elf/`)
- Type-safe representations of ELF components:
  - `Header` - ELF file header
  - `Section` - Section headers and data
  - `Symbol` - Symbol table entries
  - `Relocation` - Relocation entries
  - `ProgramHeader` - Program headers for executable

### 3. Linker Core (`src/linker/`)
- Main linking logic with key phases:
  1. **Symbol Resolution** (`resolve_symbols()`)
     - Collects symbols from all object files
     - Detects duplicates and unresolved symbols
     - Handles symbol binding (local, global, weak)
  
  2. **Section Layout** (`layout_sections()`)
     - Merges .text sections from all objects
     - Merges .data sections from all objects
     - Creates symbol table and string tables
     - Assigns virtual addresses starting at BASE_ADDR (0x400000)
  
  3. **Relocation Processing** (`apply_relocations()`)
     - Currently supports AARCH64_ADR_PREL_LO21 relocations
     - Updates instruction encodings with resolved addresses
  
  4. **Output Generation** (`write_executable()`)
     - Creates ELF header
     - Generates program headers for loading
     - Writes all sections to output file

## Memory Layout
```
0x400000 (BASE_ADDR)
├── ELF Header (64 bytes)
├── Program Headers
├── .text section (executable code)
├── .data section (initialized data)
├── .symtab (symbol table)
├── .strtab (string table)
└── .shstrtab (section name string table)
```

## Key Design Decisions
- **Single-pass linking**: Processes all objects in one pass
- **Static linking only**: No support for dynamic libraries
- **ARM64 specific**: Hardcoded for aarch64 architecture
- **Educational focus**: Clarity over performance/features