# DeepSeek Reasoner Configuration Guide

## Overview

Morgan Code has native support for DeepSeek's reasoning models, including the powerful `deepseek-reasoner` model that provides transparent reasoning processes.

## Quick Setup

1. Get your API key from [DeepSeek Platform](https://platform.deepseek.com/)

2. Set the environment variable:
```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

3. Initialize Morgan Code (if not already done):
```bash
morgan init
```

The default configuration already uses DeepSeek Reasoner!

## Configuration

Your `~/.morgan-code/config.toml` should look like this:

```toml
[llm]
provider = "deepseek"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"
temperature = 0.7
max_tokens = 4096

[llm.deepseek]
base_url = "https://api.deepseek.com/v1"
```

## Available Models

- `deepseek-reasoner` - Advanced reasoning model with transparent thought process
- `deepseek-chat` - Fast chat model for general conversations

## Features

### Reasoning Content Display

When using `deepseek-reasoner`, Morgan automatically displays the model's reasoning process:

```
You: How can I optimize this function?

Morgan: [Reasoning]
Let me analyze the function step by step:
1. First, I'll identify the bottlenecks...
2. Then consider algorithmic improvements...
3. Finally, suggest specific optimizations...

[Answer]
Here are three ways to optimize your function:
1. Use memoization to cache results...
2. Replace the nested loop with...
3. Consider using a more efficient data structure...
```

### Tool Integration

DeepSeek Reasoner works seamlessly with all Morgan Code tools:
- File operations (read, write, edit)
- Code search (glob, grep)
- Shell command execution

The model will reason about which tools to use and how to accomplish your task.

## Example Usage

### Code Analysis
```
You: Analyze the error handling in src/error.rs
Morgan: [Uses read tool, then provides analysis with reasoning]
```

### Refactoring
```
You: Refactor the Agent struct to use async streams
Morgan: [Reasons about the changes needed, uses read/edit tools]
```

### Debugging
```
You: Find all TODO comments in the codebase
Morgan: [Uses grep tool with reasoning about search strategy]
```

## Performance Tips

1. **Temperature**: Lower values (0.3-0.5) for more focused reasoning, higher (0.7-1.0) for creative solutions
2. **Max Tokens**: Increase for complex reasoning tasks (8192 or 16384)
3. **Iterations**: Adjust `max_iterations` in config for multi-step tasks

## Switching Between Models

Edit your config to switch models:

```toml
# For reasoning tasks
model = "deepseek-reasoner"

# For quick responses
model = "deepseek-chat"
```

## Cost Optimization

DeepSeek models are significantly more cost-effective than alternatives:
- Reasoning model: ~$0.55/M input tokens, ~$2.19/M output tokens
- Chat model: ~$0.14/M input tokens, ~$0.28/M output tokens

## Troubleshooting

### "API key not found"
```bash
# Make sure the environment variable is set
echo $DEEPSEEK_API_KEY

# If empty, set it:
export DEEPSEEK_API_KEY=your-key
```

### "Rate limit exceeded"
DeepSeek has generous rate limits, but if you hit them:
- Wait a few seconds between requests
- Consider using `deepseek-chat` for simpler tasks

### "Model not found"
Ensure you're using the correct model name:
- `deepseek-reasoner` (not `deepseek-r1`)
- `deepseek-chat` (not `deepseek-v2`)

## Advanced Configuration

### Custom Base URL
If using a proxy or custom endpoint:
```toml
[llm.deepseek]
base_url = "https://your-proxy.com/v1"
```

### Multiple Profiles
Create different config files for different use cases:
```bash
# Reasoning-focused
cp ~/.morgan-code/config.toml ~/.morgan-code/config-reasoning.toml

# Speed-focused
cp ~/.morgan-code/config.toml ~/.morgan-code/config-fast.toml
# Edit to use deepseek-chat
```

## Comparison with Other Providers

| Feature | DeepSeek Reasoner | OpenAI GPT-4 | Claude |
|---------|------------------|--------------|---------|
| Reasoning Display | ✅ Native | ❌ | ❌ |
| Tool Calling | ✅ | ✅ | ✅ |
| Cost | 💰 Low | 💰💰💰 High | 💰💰 Medium |
| Speed | ⚡ Fast | ⚡ Medium | ⚡ Fast |
| Context Window | 64K | 128K | 200K |

## Best Practices

1. **Use reasoning for complex tasks**: Let the model think through multi-step problems
2. **Review reasoning output**: Learn from the model's thought process
3. **Adjust temperature**: Lower for code, higher for brainstorming
4. **Leverage tools**: The model excels at deciding when to use tools

## Support

For DeepSeek-specific issues:
- [DeepSeek Documentation](https://platform.deepseek.com/docs)
- [DeepSeek API Status](https://status.deepseek.com/)

For Morgan Code issues:
- [GitHub Issues](https://github.com/your-repo/morgan-code/issues)
