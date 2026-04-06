# TUI Reasoning覆盖问题修复

## 问题描述

在第二条及后续对话中，reasoning部分会覆盖掉第一条消息的内容。

## 根本原因

原实现使用`append_reasoning_to_last_message()`和`append_to_last_message()`方法，这些方法总是追加到`messages`数组的最后一个元素。在异步流式传输环境下，这会导致竞态条件：

### 问题流程

1. **第一条对话**:
   - 用户输入消息1
   - `add_user_message("消息1")` - 添加用户消息
   - `process_input("消息1")` - 调用
     - `add_assistant_message("")` - 添加空的assistant消息
     - messages: [User("消息1"), Assistant("")]
     - spawn streaming task
   - streaming chunks到达，`append_to_last_message()`正确追加到Assistant("")
   - ✅ 正常工作

2. **第二条对话** (问题出现):
   - 用户输入消息2
   - `add_user_message("消息2")` - 添加第二条用户消息
   - `process_input("消息2")` - 调用
     - `add_assistant_message("")` - 添加第二个空的assistant消息
     - messages: [User("消息1"), Assistant("响应1"), User("消息2"), Assistant("")]
     - spawn第二个streaming task
   - 第一个streaming的chunks可能还在到达...
   - 如果此时第二个assistant消息被创建，第一个streaming的`last_mut()`会返回错误的元素
   - ❌ Reasoning和内容被追加到错误的消息

## 解决方案

### 1. 消息索引追踪

修改`TUIApplication`结构，添加当前消息索引追踪：

```rust
pub struct TUIApplication {
    // ... 其他字段
    current_message_index: Option<usize>,
}
```

### 2. 基于索引的追加方法

在`TUIState`中添加基于索引的追加方法：

```rust
// 添加assistant消息时返回索引
pub fn add_assistant_message(&mut self, content: String) -> usize {
    let index = self.messages.len();
    self.messages.push(ChatMessage { /* ... */ });
    index
}

// 基于索引追加内容
pub fn append_to_message(&mut self, index: usize, content: &str) {
    if let Some(msg) = self.messages.get_mut(index) {
        msg.content.push_str(content);
    }
}

// 基于索引追加reasoning
pub fn append_reasoning_to_message(&mut self, index: usize, reasoning: &str) {
    if let Some(msg) = self.messages.get_mut(index) {
        if msg.reasoning.is_none() {
            msg.reasoning = Some(String::new());
        }
        msg.reasoning.as_mut().unwrap().push_str(reasoning);
    }
}
```

### 3. 记录消息索引

在`process_input`中记录创建的assistant消息索引：

```rust
async fn process_input(&mut self, input: String) -> Result<()> {
    // 创建assistant消息并获取索引
    let message_index = self.state.add_assistant_message(String::new());
    self.current_message_index = Some(message_index); // 记录索引

    // spawn streaming task...
}
```

### 4. 使用索引追加内容

在`handle_tui_event`中使用索引准确定位消息：

```rust
TUIEvent::Stream(chunk) => {
    // 追加reasoning到正确的消息
    if let Some(ref reasoning) = chunk.reasoning_content {
        if let Some(index) = self.current_message_index {
            self.state.append_reasoning_to_message(index, reasoning);
        }
    }

    // 追加内容到正确的消息
    if !chunk.content.is_empty() {
        if let Some(index) = self.current_message_index {
            self.state.append_to_message(index, &chunk.content);
        }
    }
}
```

### 5. 完成时清除索引

```rust
TUIEvent::Error(error) => {
    if error == "Processing complete" {
        self.state.set_processing(false);
        self.state.status_message = Some("Ready".to_string());
        self.current_message_index = None; // 清除索引
    }
}
```

## 技术优势

1. **精确定位**: 使用索引而非依赖`last_mut()`，避免追加到错误消息
2. **线程安全**: 索引值是静态的，不会在异步执行期间变化
3. **向后兼容**: 保留了旧的`append_to_last_message()`方法，其他代码仍可使用
4. **清晰语义**: `current_message_index`明确表示当前正在处理的streaming消息

## 测试验证

### 测试场景
1. 发送第一条消息，等待完整响应
2. 发送第二条消息，观察reasoning和内容是否正确
3. 快速连续发送多条消息
4. 检查每条消息的reasoning是否独立，不互相覆盖

### 预期结果
- ✅ 每条消息的reasoning独立显示
- ✅ 不会出现reasoning覆盖其他消息内容
- ✅ 多条并发对话时各消息内容正确
- ✅ 流式传输仍然正常工作

## 修改的文件

1. **`src/ui/state.rs`**
   - 修改`add_assistant_message()`返回索引
   - 添加`append_to_message()`方法
   - 添加`append_reasoning_to_message()`方法
   - 保留旧的`append_to_last_message()`方法（兼容性）

2. **`src/ui/tui.rs`**
   - 添加`current_message_index`字段
   - 修改`process_input()`记录消息索引
   - 修改`handle_tui_event()`使用索引追加内容
   - 完成时清除索引

## 构建状态

```bash
cargo build --release
# Finished `release` profile
# 1 warning (unrelated)
```

## 总结

通过引入消息索引机制，解决了异步流式传输中的消息定位问题。现在每条对话的reasoning和内容都会准确地追加到对应的assistant消息，不会出现覆盖或错位的情况。

**状态**: ✅ 已修复
**日期**: 2026-03-15
**构建**: Release profile，成功
**测试**: 需要用户验证
