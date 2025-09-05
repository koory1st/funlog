# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-09

### Added
- New `ConfigError` type for better error handling
- `LogTemplate` struct for cleaner template generation logic
- Improved parameter value preservation for end-position logging

### Changed
- **BREAKING**: `ConfigBuilder` methods now return `Result<(), ConfigError>` instead of panicking
- Improved error messages with more descriptive context
- Enhanced parameter formatting using `{:?}` for better debug output
- Refactored complex pattern matching logic into dedicated structures

### Fixed
- Fixed move semantics issues where parameters couldn't be used in logs after being moved
- Resolved all clippy warnings
- Fixed parameter value handling for different output positions
- Improved memory efficiency by only saving parameter values when needed

### Removed
- Removed `panic!` calls in favor of proper error handling
- Cleaned up unused code and redundant logic

## [0.1.3] - Previous Release
- Initial stable release with basic functionality