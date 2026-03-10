# Morgan Code 使用示例

## 基本使用

### 1. 初始化配置

```bash
morgan init
```

这将创建 `~/.morgan-code/config.toml` 并使用默认设置。

### 2. 设置 API 密钥

```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

或者使用 OpenAI：
```bash
export OPENAI_API_KEY=your-api-key-here
```

### 3. 启动交互式聊天

```bash
morgan chat
```

## 示例对话

### 示例 1：文件操作

```
You: 读取 Cargo.toml 文件
Morgan: [使用 read 工具显示文件内容]

You: 创建一个名为 hello.rs 的新文件，包含一个简单的 hello world 程序
Morgan: [使用 write 工具创建文件]

You: 查找 src 目录中的所有 Rust 文件
Morgan: [使用 glob 工具，模式为 "src/**/*.rs"]
```

### 示例 2：代码搜索

```
You: 在代码库中搜索所有 "LLMProvider" 的出现
Morgan: [使用 grep 工具查找匹配项]

You: 显示 Agent 结构体的实现
Morgan: [使用 read 工具显示文件]
```

### 示例 3：Shell 命令

```
You: 运行 cargo test
Morgan: [使用 shell 工具执行命令]

You: 当前的 git 状态是什么？
Morgan: [使用 shell 工具运行 git status]
```

## 配置选项

### 使用不同的 LLM 提供商

#### DeepSeek Reasoner（默认）
```toml
[llm]
provider = "deepseek"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"

[llm.deepseek]
base_url = "https://api.deepseek.com/v1"
```

DeepSeek Reasoner 会在最终答案之前自动显示推理内容，帮助你理解模型的思考过程。

#### OpenAI
```toml
[llm]
provider = "openai"
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"

[llm.openai]
base_url = "https://api.openai.com/v1"
```

#### Anthropic（即将推出）
```toml
[llm]
provider = "anthropic"
model = "claude-3-opus-20240229"
api_key_env = "ANTHROPIC_API_KEY"

[llm.anthropic]
base_url = "https://api.anthropic.com/v1"
version = "2023-06-01"
```

#### Azure OpenAI（即将推出）
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

### 自定义工具行为

```toml
[tools]
enabled = ["read", "write", "edit", "glob", "grep", "shell"]
shell_timeout_seconds = 120  # shell 命令超时时间
```

### Agent 配置

```toml
[agent]
max_iterations = 50  # 最大工具调用迭代次数
enable_background_tasks = true
```

### UI 偏好设置

```toml
[ui]
show_spinner = true  # 显示加载动画
color_output = true  # 启用彩色输出
```

## 命令

- `morgan chat` - 启动交互式聊天（默认）
- `morgan init` - 创建配置文件
- `morgan config` - 显示当前配置

### 聊天中的命令

- `clear` - 清除对话上下文
- `exit` 或 `quit` - 退出聊天

## 使用技巧

1. **上下文管理**：切换话题时使用 `clear` 重置对话
2. **文件路径**：始终使用绝对路径或相对于当前目录的路径
3. **Shell 命令**：注意长时间运行的命令（默认 120 秒后超时）
4. **工具使用**：Morgan 会根据你的请求自动决定使用哪些工具

## 故障排除

### "找不到配置文件"
运行 `morgan init` 创建默认配置文件。

### "环境变量未设置"
确保导出你的 API 密钥：
```bash
export DEEPSEEK_API_KEY=your-key
# 或
export OPENAI_API_KEY=your-key
```

### "工具执行错误"
检查文件权限和路径。Morgan 需要对你正在处理的文件具有读/写访问权限。

## 开发

从源码构建：
```bash
cargo build --release
```

二进制文件将位于 `target/release/morgan`。
