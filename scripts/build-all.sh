#!/bin/bash

# 构建所有平台的脚本

set -e

echo "Building all SoftBus components..."

# 构建Rust工作空间
echo "Building Rust workspace..."
cargo build --workspace --release

# 构建Windows原生代码
if [ -d "native/windows" ]; then
    echo "Building Windows native code..."
    cd native/windows
    if command -v pwsh &> /dev/null; then
        pwsh -File build.ps1
    else
        echo "PowerShell not found, skipping Windows build"
    fi
    cd ../..
fi

# 构建Linux原生代码
if [ -d "native/linux" ]; then
    echo "Building Linux native code..."
    cd native/linux
    bash build.sh
    cd ../..
fi

echo "All builds completed successfully!"
