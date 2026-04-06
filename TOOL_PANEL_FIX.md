# 工具面板显示优化

## 问题

之前工具面板存在以下问题：

1. **默认不显示详情** - `show_details`默认为`false`，用户看不到工具参数和结果
2. **计时不会停止** - 工具完成后计时器继续运行，显示"Running for X.Xs"而不是完成时间
3. **困惑的用户体验** - 无法清楚知道工具是否完成，也无法查看详细的执行信息

## 解决方案

### 1. 默认显示详情

```rust
impl ToolPanelWidget {
    pub fn new() -> Self {
        Self {
            show_details: true,  // 默认启用详情显示
        }
    }
}
```

**效果**: 工具面板默认显示：
- 参数（Parameters）
- 结果（Result）
- 状态（Status）

### 2. 工具完成后停止计时

在`ToolExecution`结构中添加`end_time`字段：

```rust
#[derive(Debug, Clone)]
pub struct ToolExecution {
    pub name: String,
    pub status: ToolStatus,
    pub start_time: std::time::Instant,
    pub end_time: Option<std::time::Instant>,  // 新增：结束时间
    pub parameters: Option<String>,
    pub result: Option<String>,
}

impl ToolExecution {
    pub fn update_result(&mut self, result: String, success: bool) {
        self.result = Some(result);
        self.end_time = Some(std::time::Instant::now());  // 记录结束时间
        self.status = if success {
            ToolStatus::Success
        } else {
            ToolStatus::Error
        };
    }

    pub fn duration(&self) -> std::time::Duration {
        if let Some(end) = self.end_time {
            // 工具完成：返回实际持续时间
            end.duration_since(self.start_time)
        } else {
            // 工具运行中：返回已过时间
            self.start_time.elapsed()
        }
    }
}
```

### 3. 更新状态显示

修改工具面板的状态文本：

```rust
let duration_text = if tool.status == ToolStatus::Running {
    format!("Running for {:.1}s", tool.duration().as_secs_f64())
} else {
    // 工具完成：显示完成时间（而不是"Running for"）
    format!("Took {:.1}s", tool.duration().as_secs_f64())
};
```

### 4. 添加详情切换快捷键

添加'd'键来切换工具详情显示：

```rust
// Toggle tool details (d for details)
(KeyCode::Char('d'), KeyModifiers::NONE) => {
    self.renderer.tool_panel_widget().toggle_details();
}
```

更新帮助文本：

```rust
Line::from(vec![
    Span::styled("  d", Style::default().fg(Color::White)),
    Span::raw(" - "),
    Span::styled("Toggle tool details", Style::default().fg(Color::Gray)),
]),
```

## 显示效果对比

### 修改前

```
┌─ Tools ───────────────┐
│ ⏳ read_file.rs        │
│ Running for 3.5s       │  ← 计时器不会停止
│                        │
│ ⏳ grep_pattern.rs     │
│ Running for 2.1s       │
└──────────────────────────┘
```

**问题**：
- 看不到参数和结果
- 计时器持续运行，即使工具已完成

### 修改后

```
┌─ Tools ───────────────┐
│ ✓ read_file.rs        │
│ Took 3.2s             │  ← 工具完成后计时停止
│ Parameters:              │
│   file: "src/main.rs" │  ← 默认显示详情
│   offset: 10            │
│ Result:                  │
│   Success! Found file     │
│                        │
│ ⏳ grep_pattern.rs     │
│ Running for 0.5s       │
└──────────────────────────┘
```

**优势**：
- 默认显示所有工具详情
- 工具完成后计时准确停止
- 清晰的状态指示
- 可以按'd'键隐藏详情（节省空间）

## 快捷键

| 按键 | 功能 |
|--------|------|
| `Tab` | 切换工具面板显示/隐藏 |
| `d` | 切换工具详情显示/隐藏 |
| `Ctrl+C` / `Ctrl+D` | 退出 |
| `Enter` | 提交输入 |
| `?` | 切换帮助 |

## 技术细节

### 时间计算

```rust
pub fn duration(&self) -> std::time::Duration {
    if let Some(end) = self.end_time {
        // 精确计算：结束时间 - 开始时间
        end.duration_since(self.start_time)
    } else {
        // 运行中：返回已过时间
        self.start_time.elapsed()
    }
}
```

### 工具状态流程

1. **开始** (`ToolCallStart`)
   ```rust
   let tool = ToolExecution::new(name);
   state.active_tools.push(tool);
   ```
   - `status = ToolStatus::Running`
   - `end_time = None`

2. **完成** (`ToolCallEnd`)
   ```rust
   if let Some(tool) = state.active_tools.iter_mut()
       .find(|t| t.name == *name) {
       tool.update_result(result.clone(), *success);
   }
   ```
   - `status = ToolStatus::Success` 或 `ToolStatus::Error`
   - `end_time = Some(Instant::now())`

3. **显示**
   - 运行中：`Running for X.Xs`
   - 已完成：`Took X.Xs`

### 修改的文件

- `src/ui/state.rs`
  - `ToolExecution`结构：添加`end_time`字段
  - `update_result`方法：设置结束时间
  - `duration`方法：使用`end_time`计算

- `src/ui/widgets/tool_panel.rs`
  - `ToolPanelWidget::new()`：默认`show_details = true`
  - `render_tool()`方法：更新状态显示文本

- `src/ui/tui.rs`
  - `handle_key_event()`方法：添加'd'键切换详情

- `src/ui/widgets/status.rs`
  - `render_help()`方法：添加'd'键说明

## 使用示例

```bash
# 1. 运行TUI
./target/release/morgan chat --tui

# 2. 发送一个使用工具的请求
You: 列出src目录下的所有.rs文件

# 3. 观察工具面板
# - 工具运行时：显示 "Running for X.Xs"
# - 工具完成时：显示 "Took X.Xs"（计时停止）
# - 默认显示参数和结果

# 4. 按'd'键可以隐藏详情节省空间
```

## 测试验证

### 测试用例1: 工具完成时间

1. 发送一个使用shell工具的请求
2. 观察工具状态从"Running for..."变为"Took X.Xs"
3. 确认计时在工具完成后停止增加

### 测试用例2: 默认显示详情

1. 发送一个使用read工具的请求
2. 观察工具面板显示参数和结果
3. 确认默认情况下详情可见

### 测试用例3: 切换详情

1. 观察默认显示详情
2. 按'd'键隐藏详情
3. 再次按'd'键显示详情
4. 确认切换功能正常

## 已知限制

1. **工具调用链** - 同时执行多个工具时，每个工具独立计时
2. **历史记录** - 完成后的工具会保留在面板中
3. **空间限制** - 工具面板有固定宽度，详情多时可能需要滚动

## 未来改进

- [ ] 工具分组（按类型：文件操作、搜索、shell等）
- [ ] 工具性能统计（平均执行时间、成功率等）
- [ ] 工具结果预览（限制长度，支持展开查看完整）
- [ ] 工具参数语法高亮
- [ ] 并发工具的顺序可视化

---

**版本**: 1.0.0
**日期**: 2026-03-15
**状态**: ✅ 已完成
**构建**: Release profile，成功
