# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-01-09

### Added
- **人性化错误提示**: 全面改进了错误信息的用户体验
  - 中文错误信息，更易理解
  - 每个错误都包含 💡 提示，告诉用户如何修复
  - 智能拼写建议，当用户输入错误属性时自动建议正确选项
  - 冲突检测，明确指出不兼容的配置组合
- 新增错误类型：
  - `ConflictingOptions`: 检测配置冲突
  - `InvalidParameterSyntax`: 参数语法错误
  - `MissingFunction`: 宏使用位置错误
- 智能建议算法：使用编辑距离算法提供拼写建议
- 上下文感知错误：根据函数参数提供定制化建议

### Changed
- **BREAKING**: 扩展了 `ConfigError` 枚举，添加了更多具体的错误类型
- 改进了错误信息格式，使用表情符号和结构化提示
- 增强了参数验证逻辑，提供更准确的错误定位
- 优化了错误处理流程，减少了模糊的错误信息

### Fixed
- 修复了重复配置检测不准确的问题
- 改进了参数名验证的错误提示
- 修复了拼写错误时缺少建议的问题

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