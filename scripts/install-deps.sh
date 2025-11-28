#!/bin/bash

# 安装依赖的脚本

set -e

echo "Installing dependencies for SoftBus..."

# 检测操作系统
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Detected Linux system"
    
    # 安装Rust（如果未安装）
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    fi
    
    # 安装Linux依赖
    echo "Installing Linux dependencies..."
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        cmake \
        pkg-config \
        libbluetooth-dev \
        libdbus-1-dev
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS system"
    
    # 安装Rust（如果未安装）
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    fi
    
    # 安装macOS依赖
    if command -v brew &> /dev/null; then
        echo "Installing macOS dependencies via Homebrew..."
        brew install cmake pkg-config
    else
        echo "Homebrew not found. Please install Homebrew first."
        exit 1
    fi
    
else
    echo "Unsupported operating system: $OSTYPE"
    echo "Please install dependencies manually."
    exit 1
fi

echo "Dependencies installed successfully!"
