#!/bin/bash

# Test streaming implementation
echo "Testing Morgan Code streaming..."
echo ""

# Check if API key is set
if [ -z "$DEEPSEEK_API_KEY" ]; then
    echo "Warning: DEEPSEEK_API_KEY not set. Streaming test will fail."
    echo "Set it with: export DEEPSEEK_API_KEY=your-key"
    exit 1
fi

# Simple test query
echo "Running test query: 'Hello, can you introduce yourself briefly?'"
echo ""

echo "Hello, can you introduce yourself briefly?" | ./target/release/morgan chat

echo ""
echo "Test complete!"
