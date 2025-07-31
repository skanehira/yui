# Task Completion Checklist

When completing any development task in the Yui project, follow these steps:

## 1. Code Quality Checks
```bash
# Format the code
cargo fmt

# Run clippy linter and fix any warnings
cargo clippy

# Ensure no compiler warnings
cargo build
```

## 2. Run Tests
```bash
# Run all unit tests
cargo nextest run

# Run E2E tests if changes affect linking behavior
cd tests && bash run.sh
```

## 3. Verify Functionality
```bash
# If changes affect the linker output, test with example files
make build
make run  # Should output: 11

# Examine the linked executable if needed
make dump
```

## 4. Before Committing
- [ ] All tests pass (`cargo nextest run`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] No compiler warnings (`cargo build`)
- [ ] Changes are focused and atomic
- [ ] Commit message follows TDD principles:
  - Prefix with `[STRUCTURAL]` for refactoring/reorganization
  - Prefix with `[BEHAVIORAL]` for feature/bug fixes

## 5. Additional Checks for Significant Changes
- [ ] Update documentation if API changes
- [ ] Add tests for new functionality
- [ ] Consider adding benchmarks for performance-critical code
- [ ] Verify changes work on ARM64 target architecture

## Important Notes
- **Never mix structural and behavioral changes** in the same commit
- **Write tests first** when adding new features (TDD approach)
- **Use ghost** for any background processes during development