# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-XX

### Added
- Initial release of funlog procedural macro
- Support for multiple log levels: trace, debug, info, warn, error, print
- Flexible parameter logging options: all, none, specific parameters
- Position control: onStart, onEnd, onStartEnd
- Return value logging with retVal option
- Zero runtime overhead in release builds
- Comprehensive test suite with 25+ test cases
- Rich examples demonstrating all features
- Complete documentation in English and Chinese

### Features
- **Log Levels**: Support for standard log levels plus print mode
- **Parameter Control**: Choose which parameters to log
- **Position Control**: Control when logging occurs in function execution
- **Return Values**: Optional return value logging
- **Debug Only**: Macro only active in debug builds
- **Easy Integration**: Simple attribute macro syntax

### Documentation
- Complete README in English and Chinese
- Comprehensive examples directory
- API documentation
- Contributing guidelines
- Security audit workflows

### Testing
- 19 active test cases covering all functionality
- Cross-platform testing (Linux, Windows, macOS)
- Multiple Rust version support (stable, beta, nightly)
- Automated CI/CD pipeline
- Code coverage reporting

[Unreleased]: https://github.com/koory1st/funlog/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/koory1st/funlog/releases/tag/v0.1.0