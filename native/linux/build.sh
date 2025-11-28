#!/bin/bash

# Linux平台构建脚本

echo "Building SoftBus.Native for Linux..."

# 创建构建目录
mkdir -p build
cd build

# 运行CMake
cmake .. -DCMAKE_BUILD_TYPE=Release

# 编译
make -j$(nproc)

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi
