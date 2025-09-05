#!/bin/bash

# Script for testing funlog macro error messages
# This script runs various error examples to demonstrate improved error messages

echo "=========================================="
echo "Funlog Error Message Testing"
echo "=========================================="
echo ""

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test function
test_error_example() {
    local example_name=$1
    local description=$2
    
    echo -e "${BLUE}Test: ${example_name}${NC}"
    echo -e "${YELLOW}Description: ${description}${NC}"
    echo "----------------------------------------"
    
    # 运行 cargo check 并捕获输出
    cargo check --example "$example_name" 2>&1 | head -20
    
    echo ""
    echo "=========================================="
    echo ""
}

# 1. Test duplicate log level configuration
test_error_example "error_test_duplicate_log_levels" "Test error messages for duplicate log level configuration"

# 2. Test invalid parameter names
test_error_example "error_test_invalid_parameters" "Test error messages for invalid parameter names"

# 3. Test spelling mistakes
test_error_example "error_test_spelling_mistakes" "Test error messages and suggestions for spelling mistakes"

# 4. Test configuration conflicts
test_error_example "error_test_conflicting_options" "Test error messages for configuration conflicts"

# 5. Test syntax errors
test_error_example "error_test_syntax_errors" "Test error messages for syntax errors"

# 6. Test edge cases
test_error_example "error_test_edge_cases" "Test error messages for edge cases"

echo -e "${GREEN}All error message tests completed!${NC}"
echo ""
echo "Notes:"
echo "- All examples should produce compilation errors, this is expected behavior"
echo "- Focus on the quality and usefulness of error messages"
echo "- Check if useful suggestions and fix hints are provided"
echo "- Verify that error messages are clear and understandable"