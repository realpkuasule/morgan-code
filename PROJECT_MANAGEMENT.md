# Morgan Code 项目管理系统

## 概述

Morgan Code现在支持区分工具自身的代码/数据与用户项目的代码/数据，避免两者混淆。

## 主要功能

### 1. 项目根目录检测

Morgan Code会自动检测项目根目录，通过查找常见的项目标记：

- `Cargo.toml` - Rust项目
- `package.json` - Node.js项目
- `pom.xml` - Maven项目
- `build.gradle` - Gradle项目
- `requirements.txt` - Python项目
- `setup.py` - Python项目
- `pyproject.toml` - Python项目
- `go.mod` - Go项目
- `.git` - Git仓库
- `Makefile` - Make项目
- `CMakeLists.txt` - CMake项目

### 2. 文件来源标记

每个文件会被标记来源类型：

| 来源类型 | 标签 | 说明 |
|---------|------|------|
| `MorganCode` | 🔧[Morgan] | Morgan Code工具自身的文件 |
| `Project` | 📁[Project] | 用户项目目录中的文件 |
| `System` | 🖥️[System] | 系统目录文件（/usr、/etc等） |
| `Unknown` | ❓[Unknown] | 未知来源的文件 |

### 3. 配置选项

```toml
[project]
# 项目根目录（默认：当前工作目录）
project_root = "/path/to/your/project"

# Morgan Code主目录（用于配置、缓存、历史记录）
morgan_home = "/path/to/morgan/home"

# 自动检测项目根目录
auto_detect_root = true

# 显示文件来源标签
show_file_origin = true
```

## 目录结构

### Morgan Code 目录
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
/path/to/user/project/       # 用户项目（工作目录）
├── src/                    # 源代码
├── tests/                   # 测试文件
├── docs/                    # 文档
└── ...                      # 用户文件

Morgan Code在当前目录运行时，该目录即为项目根目录
```

## 使用示例

### 基本使用

```bash
# 在任意目录运行Morgan Code
cd /path/to/your/project
./target/release/morgan chat --tui

# Morgan Code会自动：
# 1. 检测项目根目录
# 2. 标记文件来源
# 3. 在工具调用中显示来源标签
```

### 配置项目根目录

```bash
# 编辑配置文件
vim ~/.morgan-code/config.toml

# 设置项目根目录
[project]
project_root = "/path/to/your/project"
```

### 禁用文件来源显示

```bash
# 编辑配置文件
vim ~/.morgan-code/config.toml

# 禁用来源标签
[project]
show_file_origin = false
```

## 工具集成

文件来源信息已集成到工具系统中：

### Read 工具
```
🔧[Morgan] src/config/types.rs
📁[Project] src/main.rs
```

### Write 工具
```
📁[Project] new_file.rs
```

### Edit 工具
```
📁[Project] src/utils.rs
```

### Glob/Grep 工具
```
📁[Project] src/**/*.rs
```

## API 使用

### 在代码中使用

```rust
use morgan_code::project::{ProjectManager, FileOrigin};

// 创建项目管理器
let morgan_home = std::path::PathBuf::from("~/.morgan-code");
let project_root = std::env::current_dir()?;
let manager = ProjectManager::new(project_root, morgan_home);

// 获取文件信息
let file_info = manager.get_file_info(&path);

// 格式化路径显示
let display_path = manager.format_path(&path, true);
println!("{}", display_path);  // 📁[Project] src/main.rs

// 获取相对路径
let relative_path = manager.get_relative_path(&path);
```

### 在工具中使用

```rust
impl Tool for ReadTool {
    fn execute(&self, args: ToolArgs) -> Result<ToolResult> {
        let path = args.get("path")?;
        let project_manager = self.project_manager.as_ref().unwrap();

        // 格式化路径显示来源
        let display_path = project_manager.format_path(path, true);
        println!("Reading: {}", display_path);

        // 读取文件
        let content = std::fs::read_to_string(path)?;

        Ok(ToolResult::Success { result: content })
    }
}
```

## 配置详解

### project_root

指定项目的根目录。如果不设置，Morgan Code会使用当前工作目录作为项目根目录。

**用法**：
```toml
[project]
project_root = "/home/user/projects/my-app"
```

**默认值**: 当前工作目录

### morgan_home

Morgan Code的主目录，用于存储配置、缓存和历史记录。

**用法**：
```toml
[project]
morgan_home = "/custom/morgan/home"
```

**默认值**: `~/.morgan-code`

### auto_detect_root

是否自动检测项目根目录。如果启用，Morgan Code会向上遍历目录树寻找项目标记。

**用法**：
```toml
[project]
auto_detect_root = true
```

**默认值**: `true`

### show_file_origin

是否在文件路径显示中包含来源标签。

**用法**：
```toml
[project]
show_file_origin = true
```

**默认值**: `true`

## 最佳实践

### 1. 项目组织

```
~/projects/
├── my-webapp/           # 项目A
│   ├── src/
│   ├── tests/
│   └── Cargo.toml
├── my-api/              # 项目B
│   ├── src/
│   ├── tests/
│   └── package.json
└── shared-libs/         # 共享库
    └── ...
```

### 2. Morgan Code配置

```bash
# 全局配置
~/.morgan-code/config.toml

# 项目特定配置（可选）
/path/to/project/.morgan/config.toml
```

### 3. 工作流

```bash
# 1. 进入项目目录
cd ~/projects/my-webapp

# 2. 运行Morgan Code
morgan chat --tui

# 3. Morgan Code会：
#    - 检测到Cargo.toml，设置项目根目录
#    - 标记所有文件操作
#    - 显示清晰的文件来源
```

## 故障排除

### 问题1: 项目根目录检测不正确

**解决方案**：手动指定项目根目录
```toml
[project]
auto_detect_root = false
project_root = "/correct/project/path"
```

### 问题2: 文件来源标签不显示

**解决方案**：确保show_file_origin已启用
```toml
[project]
show_file_origin = true
```

### 问题3: 系统文件被标记为项目文件

**原因**：系统路径检测可能不准确

**解决方案**：项目根目录不应指向系统目录

## 技术细节

### 检测算法

1. 从当前目录开始
2. 检查是否存在项目标记文件
3. 如果找到，返回该目录
4. 如果未找到，向上移动到父目录
5. 重复直到找到标记或到达根目录

### 路径判断

- **Morgan Code文件**: 路径以`morgan_home`或`/root/.claude/`开头
- **项目文件**: 路径以`project_root`开头
- **系统文件**: 路径以`/usr/`、`/etc/`、`/opt/`等开头
- **未知**: 不匹配以上任何类型

## 未来改进

- [ ] 支持多项目管理（切换不同项目）
- [ ] 项目模板系统
- [ ] 项目间代码共享
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
**状态**: ✅ 已实现
