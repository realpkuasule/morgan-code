# 快速开始指南

在 2 分钟内让 Morgan Code 与 DeepSeek Reasoner 运行起来！

## 前置要求

- Rust 工具链（1.70+）
- DeepSeek API 密钥（[在此获取](https://platform.deepseek.com/)）

## 安装

### 方式 1：从源码构建

```bash
# 克隆或导航到仓库
cd morgan-code

# 构建 release 二进制
cargo build --release

# 二进制文件位于：target/release/morgan
```

### 方式 2：全局安装（可选）

```bash
cargo install --path .
# 现在可以在任何地方使用 'morgan'
```

## 设置

### 1. 初始化配置

```bash
./target/release/morgan init
```

这将创建 `~/.morgan-code/config.toml`，默认使用 DeepSeek 作为提供商。

### 2. 设置 API 密钥

```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

**提示**：将其添加到 `~/.bashrc` 或 `~/.zshrc` 以永久保存：
```bash
echo 'export DEEPSEEK_API_KEY=your-api-key-here' >> ~/.zshrc
source ~/.zshrc
```

### 3. 开始聊天！

```bash
./target/release/morgan chat
```

## 第一次对话

```
You: 你好！你能帮我了解这个项目中有哪些文件吗？

Morgan: [推理]
我需要探索项目结构。我将使用 glob 工具查找所有文件。

[使用 glob 工具，模式为 "**/*"]

Morgan: 这个项目包含：
- src/ 中的 Rust 源文件
- Cargo.toml 配置
- README.md、USAGE.md 等文档
...

You: 读取 README.md 文件

Morgan: [使用 read 工具]
[显示 README 内容]

You: exit
```

## 常用命令

### 聊天中
- `clear` - 重置对话上下文
- `exit` 或 `quit` - 退出聊天

### CLI 命令
```bash
morgan chat      # 启动交互式聊天（默认）
morgan init      # 创建配置文件
morgan config    # 显示当前配置
```

## 快速配置更改

### 切换到 OpenAI

编辑 `~/.morgan-code/config.toml`：
```toml
[llm]
provider = "openai"
model = "gpt-4"
api_key_env = "OPENAI_API_KEY"
```

然后设置 API 密钥：
```bash
export OPENAI_API_KEY=your-openai-key
```

### 调整推理详细程度

```toml
[llm]
temperature = 0.3  # 更专注的推理
# 或
temperature = 1.0  # 更有创意的推理
```

### 更改工具超时时间

```toml
[tools]
shell_timeout_seconds = 300  # 5 分钟而不是 2 分钟
```

## 使用示例

### 1. 代码分析
```
You: 在代码库中查找所有 TODO 注释
Morgan: [使用 grep 工具搜索 "TODO"]
```

### 2. 文件操作
```
You: 创建一个名为 notes.txt 的新文件，内容为 "项目想法"
Morgan: [使用 write 工具]
```

### 3. 代码搜索
```
You: 显示 src 目录中的所有 Rust 文件
Morgan: [使用 glob 工具，模式为 "src/**/*.rs"]
```

### 4. Shell 命令
```
You: 当前的 git 状态是什么？
Morgan: [使用 shell 工具运行 "git status"]
```

## 故障排除

### "找不到配置文件"
先运行 `morgan init`。

### "环境变量 DEEPSEEK_API_KEY 未设置"
```bash
export DEEPSEEK_API_KEY=your-key
```

### "找不到命令：morgan"
使用完整路径：`./target/release/morgan`

或全局安装：`cargo install --path .`

### 构建错误
```bash
# 更新 Rust
rustup update

# 清理并重新构建
cargo clean
cargo build --release
```

## 下一步

- 阅读 [USAGE_CN.md](USAGE_CN.md) 了解详细使用示例
- 查看 [DEEPSEEK_GUIDE.md](DEEPSEEK_GUIDE.md) 了解 DeepSeek 专用功能
- 参阅 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解架构细节

## 获取帮助

- 查看文档文件
- 查看 USAGE_CN.md 中的示例对话
- 在 GitHub 上提交 issue

## 最佳实践提示

1. **具体明确**："读取 src/main.rs" 比 "显示代码" 更好
2. **使用自然语言**：Morgan 理解对话式请求
3. **让它推理**：DeepSeek Reasoner 会展示思考过程
4. **链式任务**："读取文件，找到 bug，并建议修复"
5. **使用 clear**：切换话题时重置上下文

享受使用 Morgan Code！🚀
