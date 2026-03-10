# Morgan Code

一个基于 Rust 的 AI 编码助手 CLI 工具，支持自定义 LLM 配置，原生集成 DeepSeek Reasoner。

## 特性

- **流式响应**：实时输出显示，AI 生成响应时立即反馈
- **原生 DeepSeek Reasoner 支持**：内置支持 DeepSeek 推理模型，自动显示推理内容（流式模式下以灰色显示）
- **多 LLM 提供商**：支持 DeepSeek、OpenAI、Anthropic 和 Azure OpenAI
- **工具系统**：内置文件操作工具（read、write、edit、glob、grep）和 shell 执行
- **Agent 系统**：自主 agent 可以使用工具完成任务
- **交互式聊天**：REPL 风格的自然对话界面
- **配置管理**：简单的 TOML 配置

## 安装

```bash
cargo build --release
```

二进制文件将位于 `target/release/morgan`。

## 快速开始

**初次使用 Morgan Code？** 查看[快速开始指南](QUICKSTART_CN.md)，2 分钟即可上手！

### 方式一：一键启动（推荐）
```bash
# 设置 API 密钥
export DEEPSEEK_API_KEY=your-api-key-here

# 运行启动脚本
./scripts/start.sh
```

### 方式二：手动启动
1. 初始化配置：
```bash
morgan init
```

2. 设置 API 密钥：
```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

3. 开始聊天：
```bash
morgan chat
```

## 配置

配置文件位于 `~/.morgan-code/config.toml`：

```toml
[llm]
provider = "deepseek"  # 或 "openai"、"anthropic"、"azure"
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

## 可用工具

- **read**：读取文件内容
- **write**：写入文件内容
- **edit**：替换文件中的文本
- **glob**：按模式查找文件
- **grep**：在文件中搜索文本
- **shell**：执行 shell 命令

## 命令

- `morgan chat` - 启动交互式聊天会话
- `morgan init` - 创建默认配置文件
- `morgan config` - 显示当前配置
- 在聊天中输入 `clear` 重置对话上下文
- 输入 `exit` 或 `quit` 结束会话

## 架构

```
src/
├── main.rs          # CLI 入口点
├── lib.rs           # 库导出
├── error.rs         # 错误类型
├── config/          # 配置管理
├── llm/             # LLM 抽象层
├── tools/           # 工具系统
├── agent/           # Agent 实现
├── session/         # 会话上下文
└── ui/              # 用户界面组件
```

## 文档

- [快速开始指南](QUICKSTART_CN.md) - 2 分钟快速上手
- [使用指南](USAGE_CN.md) - 详细使用示例和技巧
- [DeepSeek 指南](DEEPSEEK_GUIDE.md) - DeepSeek Reasoner 专用功能
- [项目总结](PROJECT_SUMMARY.md) - 架构和设计决策
- [更新日志](CHANGELOG.md) - 版本历史和更新

## 开发状态

已实现：
- ✅ 核心配置系统
- ✅ LLM 抽象层，支持 DeepSeek Reasoner（默认）和 OpenAI
- ✅ 完整的工具系统（文件操作 + shell）
- ✅ 带工具调用循环的 Agent
- ✅ 交互式 CLI
- ✅ DeepSeek 模型的推理内容显示

即将推出：
- Anthropic 提供商实现
- Azure OpenAI 提供商实现
- 流式响应支持
- 后台任务执行
- Plan 模式
- Hooks 系统

## 许可证

MIT
