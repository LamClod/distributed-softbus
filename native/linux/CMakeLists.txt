cmake_minimum_required(VERSION 3.15)
project(SoftBusNative VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 查找依赖
find_package(PkgConfig REQUIRED)
pkg_check_modules(BLUEZ REQUIRED bluez)

# 包含目录
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${BLUEZ_INCLUDE_DIRS}
)

# 源文件
set(SOURCES
    src/wifi_direct.cpp
    src/ble_adapter.cpp
)

# 创建共享库
add_library(softbus_native SHARED ${SOURCES})

# 链接库
target_link_libraries(softbus_native
    ${BLUEZ_LIBRARIES}
)

# 设置输出目录
set_target_properties(softbus_native PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
)

# 安装规则
install(TARGETS softbus_native
    LIBRARY DESTINATION lib
)

install(DIRECTORY include/
    DESTINATION include/softbus
    FILES_MATCHING PATTERN "*.h"
)
