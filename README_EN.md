# Morgan Code

A Rust-based AI coding assistant CLI tool with customizable LLM support, featuring native DeepSeek Reasoner integration.

## Features

- **Native DeepSeek Reasoner Support**: Built-in support for DeepSeek's reasoning models with automatic reasoning content display
- **Multiple LLM Providers**: Support for DeepSeek, OpenAI, Anthropic, and Azure OpenAI
- **Tool System**: Built-in tools for file operations (read, write, edit, glob, grep) and shell execution
- **Agent System**: Autonomous agent that can use tools to accomplish tasks
- **Interactive Chat**: REPL-style interface for natural conversations
- **Configuration Management**: Easy TOML-based configuration

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/morgan`.

## Quick Start

**New to Morgan Code?** Check out the [Quick Start Guide](QUICKSTART.md) for a 2-minute setup!

1. Initialize configuration:
```bash
morgan init
```

2. Set your API key:
```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

3. Start chatting:
```bash
morgan chat
```

## Configuration

Configuration file is located at `~/.morgan-code/config.toml`:

```toml
[llm]
provider = "deepseek"  # or "openai", "anthropic", "azure"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"
temperature = 0.7
max_tokens = 4096

[llm.deepseek]
base_url = "https://api.deepseek.com/v1"

[llm.openai]
base_url = "https://api.openai.com/v1"

[tools]
enabled = ["read", "write", "edit", "glob", "grep", "shell"]
shell_timeout_seconds = 120

[agent]
max_iterations = 50
enable_background_tasks = true

[ui]
show_spinner = true
color_output = true
```

## Available Tools

- **read**: Read file contents
- **write**: Write content to files
- **edit**: Replace text in files
- **glob**: Find files by pattern
- **grep**: Search for text in files
- **shell**: Execute shell commands

## Commands

- `morgan chat` - Start interactive chat session
- `morgan init` - Create default configuration file
- `morgan config` - Show current configuration
- Type `clear` during chat to reset conversation context
- Type `exit` or `quit` to end the session

## Architecture

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library exports
├── error.rs         # Error types
├── config/          # Configuration management
├── llm/             # LLM abstraction layer
├── tools/           # Tool system
├── agent/           # Agent implementation
├── session/         # Session context
└── ui/              # User interface components
```

## Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 2 minutes
- [Usage Guide](USAGE.md) - Detailed usage examples and tips
- [DeepSeek Guide](DEEPSEEK_GUIDE.md) - DeepSeek Reasoner specific features
- [Project Summary](PROJECT_SUMMARY.md) - Architecture and design decisions
- [Changelog](CHANGELOG.md) - Version history and updates

## Development Status

Currently implemented:
- ✅ Core configuration system
- ✅ LLM abstraction with DeepSeek Reasoner (default) and OpenAI providers
- ✅ Complete tool system (file ops + shell)
- ✅ Agent with tool-calling loop
- ✅ Interactive CLI
- ✅ Reasoning content display for DeepSeek models

Coming soon:
- Anthropic provider implementation
- Azure OpenAI provider implementation
- Streaming response support
- Background task execution
- Plan mode
- Hooks system

## License

MIT
