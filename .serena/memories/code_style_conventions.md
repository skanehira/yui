# Code Style and Conventions

## Rust Style Guidelines
- **Rust Edition**: 2024 (as specified in Cargo.toml)
- **Formatting**: Enforced by `rustfmt` - always run `cargo fmt` before committing
- **Linting**: Enforced by `clippy` - address all clippy warnings
- **Naming Conventions**:
  - Snake_case for functions, variables, and modules
  - PascalCase for types (structs, enums, traits)
  - SCREAMING_SNAKE_CASE for constants
  - Prefix type parameters with 'T' (e.g., `T`, `TValue`)

## Code Organization
- **Module Structure**: Each major component has its own module (elf, parser, linker)
- **Error Handling**: Use `Result<T, Box<dyn std::error::Error>>` for error propagation
- **Type Aliases**: Use type aliases for clarity (e.g., `type Error = Box<dyn std::error::Error>`)

## Documentation
- **Module Documentation**: Use `//!` for module-level docs
- **Function Documentation**: Use `///` for public functions with:
  - Brief description
  - `# Arguments` section
  - `# Returns` section
  - `# Example` when appropriate
- **Inline Comments**: Explain "why" not "what" - code should be self-documenting

## Testing Patterns
- **Unit Tests**: Located in `#[cfg(test)] mod tests` blocks within source files
- **Test Naming**: Use descriptive names like `test_symbol_resolution`, `test_section_layout`
- **Assertions**: Use `pretty_assertions::assert_eq!` for better diffs
- **Snapshot Tests**: Use `insta` for testing parser output consistency

## ELF-Specific Conventions
- **Binary Parsing**: Use `nom` parser combinators consistently
- **Byte Order**: Little-endian (LSB) for ARM64 target
- **Type Safety**: Strong typing for ELF structures, avoid raw numeric types
- **Magic Numbers**: Define as constants (e.g., `static BASE_ADDR: u64 = 0x400000`)

## Best Practices
- **Separation of Concerns**: Clear separation between parsing, data representation, and linking logic
- **Minimal `unsafe`**: Avoid unsafe code where possible
- **Explicit over Implicit**: Be explicit about conversions and transformations
- **Test Object Files**: Keep test fixtures in `src/parser/fixtures/`