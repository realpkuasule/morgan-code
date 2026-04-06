# Morgan Code 项目管理（简化版）

## 核心理念

Morgan Code像Claude Code一样安装到固定目录，**运行在哪个文件夹就是哪个项目**。

## 目录结构

### Morgan Code 安装目录
```
~/.morgan-code/              # Morgan Code主目录
├── config.toml              # 配置文件
├── cache/                   # 缓存目录
│   ├── llm/               # LLM响应缓存
│   └── tools/              # 工具执行缓存
└── history/                 # 历史记录
    ├── conversations.json   # 对话历史
    └── commands.json       # 命令历史
```

### 用户项目目录
```
/path/to/your/project/       # 运行morgan的目录就是项目根目录
├── src/                    # 源代码
├── tests/                   # 测试文件
├── docs/                    # 文档
└── ...                      # 用户项目文件
```

## 使用方法

### 1. 安装 Morgan Code

```bash
cargo install --path ~/.local/bin morgan-code
# 或
cargo build --release && cp target/release/morgan ~/.local/bin/morgan
```

### 2. 在项目中使用

```bash
# 进入你的项目目录
cd /path/to/your/project

# 运行 Morgan Code
morgan chat --tui

# Morgan Code会自动：
# - 当前工作目录 = 项目根目录
# - 区分Morgan Code文件和项目文件
```

## 文件来源标记

文件来源会在TUI中通过图标区分：

| 来源 | 标签 | 说明 |
|------|------|------|
| 🔧 | Morgan Code | Morgan Code工具自身文件（~/.morgan-code/） |
| 📁 | 项目 | 用户项目目录中的文件 |
| 🖥️ | 系统 | 系统目录文件（/usr、/etc等） |
| ❓ | 未知 | 其他位置的文件 |

### 配置选项

```toml
[ui]
# 是否显示文件来源标签（默认：true）
# show_file_origin = false  # 禁用标签
```

## 工具调用示例

### Read 工具
```
Reading: 🔧 ~/.morgan-code/config.toml
Reading: 📁 src/main.rs
```

### Write 工具
```
Writing: 📁 new_file.rs
```

### Shell 工具
```
Running shell: 📁 (project directory)
Command: cargo build
```

## 技术实现

### ProjectManager

```rust
use morgan_code::project::ProjectManager;

// 获取项目管理器
let config = Config::load()?;
let project_manager = config.get_project_manager();

// 获取文件信息
let file_info = project_manager.get_file_info(&path);

// 格式化显示路径
let display = project_manager.format_path(&path, true);
// 输出: 📁 src/main.rs

// 获取相对路径
let relative = project_manager.get_relative_path(&path);
// 输出: src/main.rs
```

### 路径判断规则

- **Morgan Code文件**: 路径以`~/.morgan-code`或`/root/.claude/`开头
- **项目文件**: 路径以当前工作目录开头
- **系统文件**: 路径以`/usr/`、`/etc/`、`/opt/`等开头
- **绝对路径**: 路径是绝对路径（如`/tmp/file.txt`）

### API使用

```rust
impl Tool for ReadTool {
    fn execute(&self, args: ToolArgs) -> Result<ToolResult> {
        let path = args.get("path")?;
        let config = self.config.as_ref().unwrap();
        let project_manager = config.get_project_manager();

        // 格式化路径显示
        let display_path = project_manager.format_path(&path, true);
        println!("Reading: {}", display_path);

        // 读取文件
        let content = std::fs::read_to_string(&path)?;

        Ok(ToolResult::Success { result: content })
    }
}
```

## 与Claude Code的对比

| 功能 | Morgan Code | Claude Code |
|------|------------|-------------|
| 安装方式 | cargo install | npm install |
| 配置目录 | ~/.morgan-code/ | ~/.claude/ |
| 项目目录 | 当前工作目录 | 当前工作目录 |
| 文件来源标记 | 支持 | 支持 |
| TUI | ✅ | ✅ |
| REPL | ✅ | ✅ |

## 最佳实践

### 1. 项目组织

```
~/projects/
├── my-rust-app/          # Rust项目
│   ├── src/
│   ├── Cargo.toml
│   └── tests/
├── my-node-app/          # Node.js项目
│   ├── package.json
│   ├── src/
│   └── node_modules/
└── shared-libs/          # 共享库
```

### 2. 工作流

```bash
# 1. 进入项目
cd ~/projects/my-rust-app

# 2. 运行Morgan Code
morgan chat --tui

# 3. 开始工作
# Morgan Code会：
# - 检测到Cargo.toml（项目）
# - 标记src/为📁（项目）
# - 标记~/.morgan-code/为🔧（Morgan Code）
```

### 3. 配置管理

```bash
# 查看当前配置
morgan config

# 编辑配置
vim ~/.morgan-code/config.toml

# 禁用文件来源标签（可选）
[ui]
show_file_origin = false
```

## 故障排除

### 问题1: 文件来源不正确

**原因**: 项目根目录检测错误

**解决**: Morgan Code默认使用当前工作目录，无需手动配置

### 问题2: 看不到文件来源标签

**原因**: `show_file_origin`被禁用

**解决**: 在配置中启用
```toml
[ui]
show_file_origin = true
```

### 问题3: 系统文件被标记为项目文件

**原因**: 系统路径检测不准确

**解决**: 这在复杂场景下可能发生，但不影响主要使用

## 核心优势

1. **简单直接**: 运行在哪个目录就是哪个项目
2. **清晰区分**: 明确区分Morgan Code文件和项目文件
3. **无配置负担**: 无需复杂的项目配置
4. **类似Claude Code**: 符合用户期望的工具使用方式
5. **灵活**: 可以在任何目录运行，自动识别为项目

## 未来计划

- [ ] 项目模板系统
- [ ] 项目间快速切换
- [ ] 项目元数据管理（.morgan/目录）
- [ ] 依赖分析
- [ ] 项目健康检查

## 相关文档

- [TUI_IMPLEMENTATION_SUMMARY.md](TUI_IMPLEMENTATION_SUMMARY.md) - TUI实现
- [REASONING_FIX.md](REASONING_FIX.md) - Reasoning修复
- [DEEPSEEK_GUIDE.md](DEEPSEEK_GUIDE.md) - DeepSeek配置
- [USAGE_CN.md](USAGE_CN.md) - 使用指南

---

**版本**: 1.0.0
**日期**: 2026-03-15
**状态**: ✅ 简化完成
