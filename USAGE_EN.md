# Morgan Code Usage Examples

## Basic Usage

### 1. Initialize Configuration

```bash
morgan init
```

This creates `~/.morgan-code/config.toml` with default settings.

### 2. Set API Key

```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

Or for OpenAI:
```bash
export OPENAI_API_KEY=your-api-key-here
```

### 3. Start Interactive Chat

```bash
morgan chat
```

## Example Conversations

### Example 1: File Operations

```
You: Read the Cargo.toml file
Morgan: [Uses read tool to show file contents]

You: Create a new file called hello.rs with a simple hello world program
Morgan: [Uses write tool to create the file]

You: Find all Rust files in the src directory
Morgan: [Uses glob tool with pattern "src/**/*.rs"]
```

### Example 2: Code Search

```
You: Search for all occurrences of "LLMProvider" in the codebase
Morgan: [Uses grep tool to find matches]

You: Show me the implementation of the Agent struct
Morgan: [Uses read tool to show the file]
```

### Example 3: Shell Commands

```
You: Run cargo test
Morgan: [Uses shell tool to execute the command]

You: What's the current git status?
Morgan: [Uses shell tool to run git status]
```

## Configuration Options

### Using Different LLM Providers

#### DeepSeek Reasoner (Default)
```toml
[llm]
provider = "deepseek"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"

[llm.deepseek]
base_url = "https://api.deepseek.com/v1"
```

DeepSeek Reasoner automatically displays reasoning content before the final answer, helping you understand the model's thought process.

#### OpenAI
```toml
[llm]
provider = "openai"
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"

[llm.openai]
base_url = "https://api.openai.com/v1"
```

#### Anthropic (Coming Soon)
```toml
[llm]
provider = "anthropic"
model = "claude-3-opus-20240229"
api_key_env = "ANTHROPIC_API_KEY"

[llm.anthropic]
base_url = "https://api.anthropic.com/v1"
version = "2023-06-01"
```

#### Azure OpenAI (Coming Soon)
```toml
[llm]
provider = "azure"
model = "gpt-4"
api_key_env = "AZURE_OPENAI_API_KEY"

[llm.azure]
endpoint = "https://your-resource.openai.azure.com"
deployment = "gpt-4"
api_version = "2024-02-15-preview"
```

### Customizing Tool Behavior

```toml
[tools]
enabled = ["read", "write", "edit", "glob", "grep", "shell"]
shell_timeout_seconds = 120  # Timeout for shell commands
```

### Agent Configuration

```toml
[agent]
max_iterations = 50  # Maximum tool-calling iterations
enable_background_tasks = true
```

### UI Preferences

```toml
[ui]
show_spinner = true  # Show loading spinner
color_output = true  # Enable colored output
```

## Commands

- `morgan chat` - Start interactive chat (default)
- `morgan init` - Create configuration file
- `morgan config` - Show current configuration

### In-Chat Commands

- `clear` - Clear conversation context
- `exit` or `quit` - Exit the chat

## Tips

1. **Context Management**: Use `clear` to reset the conversation when switching topics
2. **File Paths**: Always use absolute paths or paths relative to your current directory
3. **Shell Commands**: Be careful with long-running commands (they will timeout after 120s by default)
4. **Tool Usage**: Morgan automatically decides which tools to use based on your request

## Troubleshooting

### "Config file not found"
Run `morgan init` to create the default configuration file.

### "Environment variable not set"
Make sure to export your API key:
```bash
export DEEPSEEK_API_KEY=your-key
# or
export OPENAI_API_KEY=your-key
```

### "Tool execution error"
Check file permissions and paths. Morgan needs read/write access to the files you're working with.

## Development

To build from source:
```bash
cargo build --release
```

The binary will be at `target/release/morgan`.
