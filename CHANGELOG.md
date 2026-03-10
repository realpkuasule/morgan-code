# Changelog

## [0.2.0] - 2024-01-XX - DeepSeek Reasoner Integration

### Added
- **Native DeepSeek Reasoner support** - Full integration with DeepSeek's reasoning models
- Automatic reasoning content display for DeepSeek models
- DeepSeek configuration in default config template
- Comprehensive DeepSeek guide (DEEPSEEK_GUIDE.md)
- Support for `deepseek-reasoner` and `deepseek-chat` models

### Changed
- **Default LLM provider changed from OpenAI to DeepSeek**
- Updated all documentation to reflect DeepSeek as the primary provider
- Enhanced LLM factory to support DeepSeek provider
- Updated configuration examples across all docs

### Technical Details
- Added `src/llm/deepseek.rs` with full DeepSeek API implementation
- Added `DeepSeekConfig` to configuration types
- Implemented reasoning content extraction and display
- Maintained backward compatibility with OpenAI provider

## [0.1.0] - 2024-01-XX - Initial Release

### Added
- Core infrastructure with error handling
- Configuration management with TOML support
- LLM abstraction layer with provider trait
- OpenAI provider implementation
- Tool system with 6 core tools:
  - Read: Read file contents
  - Write: Create/write files
  - Edit: Replace text in files
  - Glob: Find files by pattern
  - Grep: Search text in files
  - Shell: Execute shell commands
- Agent system with autonomous tool-calling loop
- Session context management
- Interactive CLI with REPL interface
- Progress indicators and spinners
- Comprehensive documentation
- Integration tests

### Technical Stack
- Rust 2021 edition
- Tokio for async runtime
- Clap for CLI parsing
- Reqwest for HTTP client
- Serde for serialization
- Tree-sitter ready for code analysis

### Architecture
- Trait-based extensible design
- Modular component structure
- Type-safe error handling
- Configuration-driven behavior
