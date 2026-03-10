# Streaming Tool Call Fixes - Complete Guide

## Issues Encountered and Fixed

### Issue 1: Missing field `file_path`
**Error:** `Invalid parameter: missing field 'file_path'`

**Root Cause:**
Tool call arguments arrive in chunks during streaming. We were trying to parse incomplete JSON immediately, which resulted in empty objects `{}` that failed validation.

**Fix:**
- Introduced `ToolCallChunk` type to carry raw string data
- Accumulate argument strings across all chunks
- Parse JSON only after stream completes

### Issue 2: Missing field `name`
**Error:** `Parse error: missing field 'name' at line 1 column 269`

**Root Cause:**
In streaming mode, tool call fields (`id`, `name`, `arguments`) arrive incrementally. The original `DeepSeekToolCall` structure required all fields to be present, causing deserialization to fail on early chunks.

**Fix:**
- Created separate streaming structures:
  - `DeepSeekStreamToolCall` with all optional fields
  - `DeepSeekStreamFunctionCall` with all optional fields
  - `OpenAIStreamToolCall` with all optional fields
  - `OpenAIStreamFunctionCall` with all optional fields
- Added `#[serde(default)]` to gracefully handle missing fields
- Updated parsing logic to handle `None` values

### Issue 3: EOF while parsing JSON
**Error:** `Failed to parse tool call arguments: EOF while parsing a string at line 1 column 15469`

**Root Cause:**
The stream was ending (finish_reason set) before all argument chunks arrived, resulting in incomplete JSON strings.

**Fixes Applied:**
1. **Don't break early on finish_reason with tool calls**
   - Continue accumulating if tool_call_chunks are present
   - Only break when no more tool call data is coming

2. **Validate JSON completeness before parsing**
   - Check if arguments string is non-empty
   - Verify it starts with `{` and ends with `}`
   - Skip parsing if incomplete

3. **Better error handling**
   - Log warnings for incomplete tool calls
   - Continue processing other tool calls if one fails
   - Fall back to regular response if no valid tool calls parsed

## Implementation Details

### Type System (`src/llm/types.rs`)
```rust
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_chunks: Vec<ToolCallChunk>,  // New field
    pub finish_reason: Option<FinishReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub name: Option<String>,
    pub arguments: Option<String>,  // Raw string, not parsed
}
```

### DeepSeek Provider (`src/llm/deepseek.rs`)
```rust
#[derive(Debug, Deserialize)]
struct DeepSeekStreamToolCall {
    #[serde(default)]
    index: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    function: Option<DeepSeekStreamFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekStreamFunctionCall {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}
```

### Agent Logic (`src/agent/agent.rs`)
```rust
// Accumulate tool call chunks
let mut tool_call_accumulator: HashMap<usize, (String, String, String)> = HashMap::new();

for tc_chunk in &chunk.tool_call_chunks {
    let entry = tool_call_accumulator.entry(tc_chunk.index).or_insert(...);
    // Append strings, don't parse yet
    if let Some(args) = &tc_chunk.arguments {
        entry.2.push_str(args);
    }
}

// Don't break early if tool calls are still coming
if chunk.finish_reason.is_some() && !chunk.tool_call_chunks.is_empty() {
    continue;
}

// After stream ends, validate and parse
for (_, (id, name, args_str)) in tool_call_accumulator {
    // Validate JSON is complete
    let trimmed = args_str.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        eprintln!("Warning: incomplete JSON");
        continue;
    }

    // Parse complete JSON
    match serde_json::from_str(&args_str) {
        Ok(parameters) => { /* use it */ }
        Err(e) => { /* log and skip */ }
    }
}
```

## Files Modified

1. **src/llm/types.rs**
   - Added `ToolCallChunk` struct
   - Added `tool_call_chunks` field to `StreamChunk`

2. **src/llm/deepseek.rs**
   - Added `DeepSeekStreamToolCall` with optional fields
   - Added `DeepSeekStreamFunctionCall` with optional fields
   - Updated streaming logic to emit chunks instead of parsing

3. **src/llm/openai.rs**
   - Added `OpenAIStreamToolCall` with optional fields
   - Added `OpenAIStreamFunctionCall` with optional fields
   - Updated streaming logic to emit chunks instead of parsing

4. **src/agent/agent.rs**
   - Added HashMap-based accumulator for tool call chunks
   - Added logic to continue streaming when tool calls are present
   - Added JSON validation before parsing
   - Added comprehensive error handling and fallback logic

## Testing

### Build Status
```bash
cargo build --release
# ✓ Compiles successfully
```

### Test Cases
1. **Simple query** - "Hello" → Should stream normally
2. **Tool call** - "Read Cargo.toml" → Should accumulate and parse correctly
3. **Multiple tool calls** - Complex tasks → Should handle all tool calls
4. **Error cases** - Incomplete data → Should log warnings and continue

### Expected Behavior
- Tool call arguments accumulate across all chunks
- JSON parsing happens only after complete data received
- Incomplete tool calls are logged and skipped
- Valid tool calls execute normally
- Fallback to regular response if no valid tool calls

## Key Insights

1. **Streaming is incremental** - Data arrives piece by piece, not all at once
2. **finish_reason timing** - May be set before last chunk arrives
3. **JSON validation** - Must verify completeness before parsing
4. **Graceful degradation** - Skip invalid tool calls, continue with valid ones
5. **Error visibility** - Log warnings to help debug issues

## Future Improvements

1. Add timeout for tool call accumulation
2. Implement retry logic for failed tool calls
3. Add metrics for tool call success/failure rates
4. Consider buffering strategy for very large arguments
5. Add unit tests for edge cases
