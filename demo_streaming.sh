#!/bin/bash

# Morgan Code - Streaming Feature Demo
# This script demonstrates the new streaming capabilities

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Morgan Code - Streaming Response Demo             ║"
echo "╔════════════════════════════════════════════════════════════╗"
echo ""

# Check prerequisites
if [ ! -f "./target/release/morgan" ]; then
    echo "❌ Morgan binary not found. Building..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

if [ -z "$DEEPSEEK_API_KEY" ]; then
    echo "⚠️  DEEPSEEK_API_KEY not set"
    echo ""
    echo "To use streaming with DeepSeek:"
    echo "  export DEEPSEEK_API_KEY=your-api-key"
    echo ""
    echo "To use streaming with OpenAI:"
    echo "  1. Edit ~/.morgan-code/config.toml"
    echo "  2. Change provider to 'openai'"
    echo "  3. export OPENAI_API_KEY=your-api-key"
    echo ""
    exit 1
fi

echo "✅ Prerequisites met"
echo ""
echo "Features implemented:"
echo "  ✓ Real-time streaming output"
echo "  ✓ DeepSeek reasoning display (gray text)"
echo "  ✓ Tool call support in streaming mode"
echo "  ✓ Both DeepSeek and OpenAI providers"
echo ""
echo "Starting interactive chat..."
echo "Try these commands:"
echo "  - 'Hello' (basic streaming)"
echo "  - 'Explain Rust ownership' (with reasoning if using deepseek-reasoner)"
echo "  - 'Read Cargo.toml' (tool call streaming)"
echo "  - 'exit' to quit"
echo ""
echo "────────────────────────────────────────────────────────────"
echo ""

./target/release/morgan chat
