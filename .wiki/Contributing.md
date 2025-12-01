# Contributing Guide

Thank you for your interest in contributing to rVsrg! This guide will help you get started.

## Code of Conduct

Be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests and lints: `cargo test && cargo clippy`
5. Submit a pull request

## Code Style

We follow standard Rust conventions:

### Formatting

```bash
# Format code before committing
cargo fmt
```

### Linting

```bash
# Check for common issues
cargo clippy --all-targets -- -W clippy::all
```

### Documentation

- Add doc comments (`///`) to public items
- Use module-level docs (`//!`) for file headers
- Write documentation in English

Example:
```rust
/// Calculates accuracy percentage from hit statistics.
///
/// # Returns
/// A value between 0.0 and 100.0 representing accuracy.
pub fn calculate_accuracy(&self) -> f64 {
    // ...
}
```

## Project Structure Guidelines

### Adding New Features

1. **Logic** goes in `src/logic/`
2. **UI Components** go in `src/views/components/`
3. **Data Models** go in `src/models/`
4. **Shaders** go in `src/shaders/`

### Thread Safety

- Use channels for inter-thread communication
- Avoid shared mutable state
- Prefer immutable snapshots for render data

### Performance

- Profile before optimizing
- Use `#[inline]` sparingly
- Avoid allocations in hot paths

## Pull Request Checklist

- [ ] Code compiles without errors
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` passes without warnings
- [ ] Documentation added for new public APIs
- [ ] Commit messages are clear and descriptive

## Issue Reporting

When reporting bugs, please include:

1. Steps to reproduce
2. Expected behavior
3. Actual behavior
4. System information (OS, GPU, etc.)
5. Relevant log output

## Feature Requests

Open an issue with:

1. Clear description of the feature
2. Use case / motivation
3. Proposed implementation (if any)

## Questions?

Feel free to open a discussion or issue if you have questions!

