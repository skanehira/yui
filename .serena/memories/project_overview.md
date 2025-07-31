# Yui Project Overview

## Purpose
Yui (結) is a toy ELF linker written in Rust. The name means "to bind" or "to connect" in Japanese. This is an educational project designed for learning about linker internals and ELF file format processing.

## Tech Stack
- **Language**: Rust (1.85.0)
- **Target Architecture**: aarch64-unknown-linux-gnu (ARM64 Linux)
- **Build System**: Cargo + Make
- **Dependencies**:
  - `nom` (8.0.0) - Parser combinator library for binary parsing
  - `pretty_assertions` (1.4.1) - Better assertion diffs in tests (dev)
  - `insta` (1.43.1) - Snapshot testing (dev)

## Project Structure
- `src/` - Main source code
  - `main.rs` - Entry point for the linker executable
  - `lib.rs` - Library root
  - `elf/` - ELF format data structures
  - `parser/` - ELF file parsing using nom
  - `linker/` - Core linking logic
- `tests/` - E2E test scripts
- `benches/` - Performance benchmarks
- `.github/workflows/` - CI/CD configuration

## Key Components
1. **Parser Module**: Parses ELF object files using nom combinators
2. **ELF Module**: Type definitions for ELF format components
3. **Linker Module**: Core linking logic including symbol resolution, section layout, and relocation

## Development Environment
- Platform: Darwin (macOS)
- Requires Rust 1.85.0
- Uses cargo-nextest for testing
- Uses cargo-llvm-cov for coverage