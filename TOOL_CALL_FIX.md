# Tool Call Streaming Fix

## Problem
When using streaming mode with tool calls, the error "Invalid parameter: missing field `file_path`" occurred. This was because tool call arguments in streaming mode arrive in chunks, and we were trying to parse incomplete JSON immediately.

## Root Cause
In the original implementation:
```rust
// DeepSeek streaming - WRONG
let tool_calls = delta.tool_calls.as_ref()
    .map(|calls| calls.iter().map(|tc| ToolCall {
        id: tc.id.clone(),
        name: tc.function.name.clone(),
        parameters: serde_json::from_str(&tc.function.arguments)
            .unwrap_or(serde_json::json!({})),  // ❌ Creates empty {} on parse failure
    }).collect())
```

When `tc.function.arguments` contained incomplete JSON like `{"file_pa`, the parse would fail and create an empty object `{}`, which then failed validation when the tool tried to execute.

## Solution
Introduced a two-phase approach:

### Phase 1: Accumulate chunks as strings
Added `ToolCallChunk` type to carry raw string data:
```rust
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub name: Option<String>,
    pub arguments: Option<String>,  // Raw string, not parsed JSON
}
```

### Phase 2: Parse only when complete
In the Agent, accumulate all chunks first, then parse:
```rust
// Accumulate during streaming
let mut tool_call_accumulator: HashMap<usize, (String, String, String)> = HashMap::new();

for tc_chunk in &chunk.tool_call_chunks {
    let entry = tool_call_accumulator.entry(tc_chunk.index).or_insert(...);
    // Append argument strings
    if let Some(args) = &tc_chunk.arguments {
        entry.2.push_str(args);
    }
}

// Parse after stream completes
for (_, (id, name, args_str)) in tool_call_accumulator {
    let parameters = serde_json::from_str(&args_str)
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to parse tool call arguments: {}", e);
            serde_json::json!({})
        });
    // Now we have complete JSON
}
```

## Files Modified
- `src/llm/types.rs` - Added `ToolCallChunk` struct and `tool_call_chunks` field to `StreamChunk`
- `src/llm/deepseek.rs` - Changed to emit `ToolCallChunk` instead of parsing immediately
- `src/llm/openai.rs` - Same change for OpenAI provider
- `src/agent/agent.rs` - Added accumulation logic for tool call chunks

## Testing
Build successful:
```bash
cargo build --release
# ✓ Compiles without errors or warnings
```

## Expected Behavior After Fix
1. Tool calls in streaming mode will accumulate argument strings
2. JSON parsing happens only after the complete argument string is received
3. No more "missing field" errors due to incomplete JSON
4. Tool execution proceeds normally with complete parameters

## Example Flow
```
Stream chunk 1: {"file_
Stream chunk 2: path": "/root/
Stream chunk 3: test.txt"}

Accumulated: {"file_path": "/root/test.txt"}
Parsed: ✓ Valid JSON with all required fields
Tool execution: ✓ Success
```
