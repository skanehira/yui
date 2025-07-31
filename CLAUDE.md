# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Yui (結) is a toy ELF linker written in Rust. The name means "to bind" or "to connect" in Japanese. This is an educational project designed for learning about linker internals and ELF file format.

## Development Commands

### Building
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Build test object files
make build-obj
# or directly:
cd src/parser/fixtures && gcc -c main.c -o main.o && gcc -c sub.c -o sub.o

# Build and link example files
make build
# Equivalent to: cargo run -- a.out src/parser/fixtures/main.o src/parser/fixtures/sub.o
```

### Testing
```bash
# Run unit tests with cargo-nextest (preferred)
cargo nextest run
# or
make test

# Run E2E tests
cd tests && bash run.sh

# Run a single test file
cargo nextest run --test <test_name>

# Run tests with coverage
cargo llvm-cov nextest --lcov --output-path lcov.info
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check

# Run clippy linter
cargo clippy

# Run benchmarks (requires nightly)
cargo +nightly bench
# or
make bench
```

### Running the Linker
```bash
# Basic usage
cargo run -- <output> <input1> [<input2> ...]

# Example with test files
make run  # Links test objects and runs the output

# Examine output
make dump  # Shows ELF headers, symbols, sections, and disassembly
```

## Architecture

### Module Structure

The codebase is organized around three main components:

1. **Parser** (`src/parser/`): Parses ELF object files using the `nom` parser combinator library
   - `header.rs`: ELF header parsing
   - `section.rs`: Section header table parsing
   - `symbol.rs`: Symbol table parsing
   - `relocation.rs`: Relocation entries parsing

2. **ELF Data Structures** (`src/elf/`): Type definitions for ELF format components
   - `header.rs`: ELF header structures
   - `section.rs`: Section definitions and types
   - `symbol.rs`: Symbol table structures
   - `relocation.rs`: Relocation types and structures
   - `segument.rs`: Segment/program header definitions

3. **Linker** (`src/linker/`): Core linking logic
   - Main linker orchestration in `linker.rs`
   - Symbol resolution
   - Section layout
   - Output file generation

### Key Design Patterns

- **Parser Combinators**: Uses `nom` for parsing binary ELF data
- **Type Safety**: Strong typing for ELF structures with minimal unsafe code
- **Separation of Concerns**: Clear separation between parsing, data representation, and linking logic

### Target Architecture

Currently targets `aarch64-unknown-linux-gnu` (ARM64 Linux) as specified in `rust-toolchain.toml`. The base virtual address for executables is set to `0x400000`.

## Testing Strategy

- **Unit Tests**: Located alongside source files for parser and ELF modules
- **Snapshot Tests**: Uses `insta` for testing parser output consistency
- **E2E Tests**: Shell scripts in `tests/` directory that compile C programs and link them
- **Test Fixtures**: C source files in `src/parser/fixtures/` for generating test object files

## Dependencies

- `nom` (8.0.0): Parser combinator library for binary parsing
- `pretty_assertions` (1.4.1): Better assertion diffs in tests
- `insta` (1.43.1): Snapshot testing

## Rust Version

Requires Rust 1.85.0 as specified in `rust-toolchain.toml`.