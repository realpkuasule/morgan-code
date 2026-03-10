# Quick Start Guide

Get Morgan Code running with DeepSeek Reasoner in under 2 minutes!

## Prerequisites

- Rust toolchain (1.70+)
- DeepSeek API key ([Get one here](https://platform.deepseek.com/))

## Installation

### Option 1: Build from Source

```bash
# Clone or navigate to the repository
cd morgan-code

# Build release binary
cargo build --release

# The binary is at: target/release/morgan
```

### Option 2: Install Globally (Optional)

```bash
cargo install --path .
# Now you can use 'morgan' from anywhere
```

## Setup

### 1. Initialize Configuration

```bash
./target/release/morgan init
```

This creates `~/.morgan-code/config.toml` with DeepSeek as the default provider.

### 2. Set Your API Key

```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

**Tip**: Add this to your `~/.bashrc` or `~/.zshrc` to make it permanent:
```bash
echo 'export DEEPSEEK_API_KEY=your-api-key-here' >> ~/.zshrc
source ~/.zshrc
```

### 3. Start Chatting!

```bash
./target/release/morgan chat
```

## Your First Conversation

```
You: Hello! Can you help me understand what files are in this project?

Morgan: [Reasoning]
I need to explore the project structure. I'll use the glob tool to find all files.

[Uses glob tool with pattern "**/*"]

Morgan: This project contains:
- Rust source files in src/
- Configuration in Cargo.toml
- Documentation in README.md, USAGE.md, etc.
...

You: Read the README.md file

Morgan: [Uses read tool]
[Displays README content]

You: exit
```

## Common Commands

### In Chat
- `clear` - Reset conversation context
- `exit` or `quit` - Exit the chat

### CLI Commands
```bash
morgan chat      # Start interactive chat (default)
morgan init      # Create config file
morgan config    # Show current configuration
```

## Quick Configuration Changes

### Switch to OpenAI

Edit `~/.morgan-code/config.toml`:
```toml
[llm]
provider = "openai"
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"
```

Then set the API key:
```bash
export OPENAI_API_KEY=your-openai-key
```

### Adjust Reasoning Verbosity

```toml
[llm]
temperature = 0.3  # More focused reasoning
# or
temperature = 1.0  # More creative reasoning
```

### Change Tool Timeout

```toml
[tools]
shell_timeout_seconds = 300  # 5 minutes instead of 2
```

## Example Use Cases

### 1. Code Analysis
```
You: Find all TODO comments in the codebase
Morgan: [Uses grep tool to search for "TODO"]
```

### 2. File Operations
```
You: Create a new file called notes.txt with "Project ideas"
Morgan: [Uses write tool]
```

### 3. Code Search
```
You: Show me all Rust files in the src directory
Morgan: [Uses glob tool with "src/**/*.rs"]
```

### 4. Shell Commands
```
You: What's the current git status?
Morgan: [Uses shell tool to run "git status"]
```

## Troubleshooting

### "Config file not found"
Run `morgan init` first.

### "Environment variable DEEPSEEK_API_KEY not set"
```bash
export DEEPSEEK_API_KEY=your-key
```

### "Command not found: morgan"
Use the full path: `./target/release/morgan`

Or install globally: `cargo install --path .`

### Build Errors
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Next Steps

- Read [USAGE.md](USAGE.md) for detailed usage examples
- Check [DEEPSEEK_GUIDE.md](DEEPSEEK_GUIDE.md) for DeepSeek-specific features
- See [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for architecture details

## Getting Help

- Check the documentation files
- Review example conversations in USAGE.md
- Open an issue on GitHub

## Tips for Best Results

1. **Be specific**: "Read src/main.rs" is better than "show me the code"
2. **Use natural language**: Morgan understands conversational requests
3. **Let it reason**: DeepSeek Reasoner shows its thought process
4. **Chain tasks**: "Read the file, find the bug, and suggest a fix"
5. **Use clear**: Reset context when switching topics

Enjoy using Morgan Code! 🚀
