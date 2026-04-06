#!/bin/bash

# Test script for TUI mode
# This script will launch the TUI and send some input to test basic functionality

echo "Testing TUI mode..."
echo "===================="

# Set a dummy API key (we're testing the UI, not the API)
export DEEPSEEK_API_KEY=test_key_for_tui_testing

# Create a test script that will send input to the TUI
(
    sleep 2
    echo "hello"
    sleep 2
    # Send Ctrl+D to quit
    echo -e "\004"
) | timeout 10 ./target/release/morgan chat --tui

echo ""
echo "TUI test completed"
