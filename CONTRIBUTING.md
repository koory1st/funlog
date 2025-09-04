# Contributing to Funlog

Thank you for your interest in contributing to Funlog! This document provides guidelines and information for contributors.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Code examples
- Environment information (OS, Rust version, funlog version)

### Suggesting Features

Feature requests are welcome! Please provide:

- A clear and descriptive title
- Detailed description of the proposed feature
- Use cases and examples
- Any alternatives you've considered

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Run clippy (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Git

### Setup

```bash
git clone https://github.com/koory1st/funlog.git
cd funlog
cargo build
cargo test
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run all examples
./run_tests.sh

# Run specific example
cargo run --example raw_debug
```

### Code Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common mistakes
- Follow Rust naming conventions
- Add documentation for public APIs
- Write tests for new functionality

## Project Structure

```
funlog/
├── src/                    # Source code
│   ├── lib.rs             # Main library entry point
│   ├── config.rs          # Configuration structures
│   ├── config_builder.rs  # Configuration builder
│   ├── generics_item_fn.rs # Function analysis
│   └── output.rs          # Code generation
├── examples/              # Usage examples
├── tests/                 # Integration tests
├── .github/               # GitHub workflows and templates
└── docs/                  # Documentation
```

## Testing Guidelines

- Write unit tests for new functions
- Add integration tests for new features
- Include examples demonstrating new functionality
- Ensure all tests pass before submitting PR

## Documentation

- Update README.md for user-facing changes
- Add inline documentation for new public APIs
- Include examples in documentation
- Update CHANGELOG.md for releases

## Release Process

Releases are automated through GitHub Actions:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.1.1`
4. Push tag: `git push origin v0.1.1`
5. GitHub Actions will automatically:
   - Run tests
   - Create GitHub release
   - Publish to crates.io

## Questions?

If you have questions about contributing, please:

1. Check existing issues and discussions
2. Create a new issue with the "question" label
3. Contact the maintainers

Thank you for contributing to Funlog!