#!/bin/bash

# 运行所有测试的脚本

set -e

echo "Running all tests..."

# 运行Rust测试
echo "Running Rust tests..."
cargo test --workspace --verbose

# 运行集成测试
echo "Running integration tests..."
cargo test --workspace --test '*' --verbose

echo "All tests passed!"
