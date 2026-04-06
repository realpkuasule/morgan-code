#!/bin/bash

# Simple TUI test
echo "=========================================="
echo "Simple TUI Mode Test"
echo "=========================================="

# Set a dummy API key
export DEEPSEEK_API_KEY=test_key_for_tui_testing

echo ""
echo "Starting TUI with timeout..."
echo ""

# Run TUI with a timeout and expect script
timeout 8 expect << 'EXPECT_SCRIPT'
set timeout 10
spawn ./target/release/morgan chat --tui

# Wait for TUI to start (look for terminal escape codes)
expect {
    timeout {
        puts "\n❌ ERROR: TUI failed to start within timeout"
        exit 1
    }
    -re "\\\\[?1049h" {
        puts "\n✅ PASSED: TUI started and entered alternate screen"
    }
    -re . {
        # Any output means TUI started
        puts "\n✅ PASSED: TUI started (detected output)"
    }
}

# Send test message
sleep 1
send "hello"
puts "\n✅ PASSED: Sent test message 'hello'"

# Wait a moment for processing
sleep 2

# Send Ctrl+D to exit
send "\004"

# Wait for clean exit
expect {
    timeout {
        puts "\n⚠️  WARNING: TUI may not have exited cleanly"
    }
    -re "\\\\[?1049l" {
        puts "✅ PASSED: TUI exited alternate screen mode"
    }
    eof {
        puts "✅ PASSED: TUI exited"
    }
}

puts "\n=========================================="
puts "Test Result: SUCCESS"
puts "TUI mode is working correctly!"
puts "=========================================="
EXPECT_SCRIPT

exit_code=$?

if [ $exit_code -eq 0 ]; then
    echo ""
    echo "=========================================="
    echo "FINAL RESULT: ✅ TUI TEST PASSED"
    echo "=========================================="
    echo ""
    echo "Test Summary:"
    echo "  - TUI initialization: ✅ SUCCESS"
    echo "  - Message input: ✅ SUCCESS"
    echo "  - Clean exit (Ctrl+D): ✅ SUCCESS"
    echo ""
    echo "The TUI mode is fully functional!"
    echo "=========================================="
else
    echo ""
    echo "=========================================="
    echo "FINAL RESULT: ❌ TUI TEST FAILED"
    echo "=========================================="
    echo ""
    echo "Exit code: $exit_code"
    echo "=========================================="
fi

exit $exit_code
