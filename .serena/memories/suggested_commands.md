# Suggested Commands for Yui Development

## Building
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

## Testing
```bash
# Run unit tests with cargo-nextest (preferred)
cargo nextest run

# Using make
make test

# Run E2E tests
cd tests && bash run.sh

# Run a single test
cargo nextest run --test <test_name>

# Run tests with coverage
cargo llvm-cov nextest --lcov --output-path lcov.info
```

## Code Quality
```bash
# Format code
cargo fmt

# Check formatting (without making changes)
cargo fmt --all -- --check

# Run clippy linter
cargo clippy

# Run benchmarks (requires nightly)
cargo +nightly bench
# or
make bench
```

## Running the Linker
```bash
# Basic usage
cargo run -- <output> <input1> [<input2> ...]

# Example with test files
make run  # Links test objects and runs the output

# Examine output
make dump  # Shows ELF headers, symbols, sections, and disassembly
```

## System Utilities (Darwin/macOS)
```bash
# File operations
ls -la        # List files with details
find . -name "*.rs"  # Find Rust files
grep -r "pattern" .  # Search for pattern (use ripgrep 'rg' for better performance)

# Git operations
git status
git diff
git add .
git commit -m "message"
git push

# Process management (using ghost for background processes)
ghost run <command>  # Run background process
ghost stop <id>      # Stop process
ghost list           # List processes
ghost log <id>       # View logs

# File inspection
readelf -a <file>    # Examine ELF files
objdump -d <file>    # Disassemble binary
hexdump -C <file>    # View hex dump
```