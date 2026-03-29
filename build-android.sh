#!/bin/bash

# 纯粹直播 Rust 版 - Android APK 构建脚本
# 使用方法：./build-android.sh

set -e

echo "🚀 纯粹直播 Rust 版 - Android 构建工具"
echo "========================================="

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查函数
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}❌ 错误：$1 未安装${NC}"
        return 1
    fi
    echo -e "${GREEN}✅ $1 已安装${NC}"
    return 0
}

# 检查环境
echo ""
echo "📋 检查环境..."
echo ""

# 检查基本工具
check_command node || exit 1
check_command npm || exit 1
check_command cargo || exit 1
check_command rustc || exit 1

# 检查 Android 环境
ANDROID_SDK_FOUND=false

if [ -n "$ANDROID_HOME" ] && [ -d "$ANDROID_HOME" ]; then
    echo -e "${GREEN}✅ ANDROID_HOME 已设置：$ANDROID_HOME${NC}"
    ANDROID_SDK_FOUND=true
elif [ -n "$ANDROID_SDK_ROOT" ] && [ -d "$ANDROID_SDK_ROOT" ]; then
    echo -e "${GREEN}✅ ANDROID_SDK_ROOT 已设置：$ANDROID_SDK_ROOT${NC}"
    ANDROID_SDK_FOUND=true
elif [ -d "$HOME/Library/Android/sdk" ]; then
    echo -e "${GREEN}✅ 找到 Android SDK: $HOME/Library/Android/sdk${NC}"
    export ANDROID_HOME=$HOME/Library/Android/sdk
    ANDROID_SDK_FOUND=true
else
    echo -e "${YELLOW}⚠️  未找到 Android SDK${NC}"
fi

# 检查 Java
if command -v java &> /dev/null; then
    JAVA_VERSION=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
    if [ "$JAVA_VERSION" -ge 17 ]; then
        echo -e "${GREEN}✅ Java $JAVA_VERSION 已安装${NC}"
    else
        echo -e "${YELLOW}⚠️  Java 版本过低 ($JAVA_VERSION)，需要 17+${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Java 未安装，需要 17+${NC}"
fi

# 检查 Rust Android 目标
echo ""
echo "📦 检查 Rust Android 目标..."
if rustup target list --installed | grep -q "aarch64-linux-android"; then
    echo -e "${GREEN}✅ aarch64-linux-android 已安装${NC}"
else
    echo -e "${YELLOW}⚠️  aarch64-linux-android 未安装${NC}"
    echo "正在安装..."
    rustup target add aarch64-linux-android || echo -e "${YELLOW}⚠️  安装失败，请手动安装${NC}"
fi

# 检查 ADB
if command -v adb &> /dev/null; then
    echo -e "${GREEN}✅ ADB 已安装${NC}"
    
    # 检查设备连接
    echo ""
    echo "📱 检查设备连接..."
    DEVICES=$(adb devices 2>&1 | grep -v "List" | grep "device" | wc -l | tr -d ' ')
    if [ "$DEVICES" -gt 0 ]; then
        echo -e "${GREEN}✅ 检测到 $DEVICES 个设备${NC}"
        adb devices
    else
        echo -e "${YELLOW}⚠️  未检测到设备连接${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  ADB 未安装${NC}"
fi

# 构建选项
echo ""
echo "🔧 构建选项"
echo "-----------"
echo "1. Debug 构建 (快速，用于测试)"
echo "2. Release 构建 (优化，用于发布)"
echo "3. 仅检查环境"
echo ""

read -p "请选择构建模式 (1/2/3): " BUILD_CHOICE

if [ "$BUILD_CHOICE" = "3" ]; then
    echo -e "${GREEN}✅ 环境检查完成${NC}"
    exit 0
fi

# 选择目标架构
echo ""
echo "🎯 选择目标架构"
echo "---------------"
echo "1. aarch64 (ARM64 - 现代设备，推荐)"
echo "2. x86_64 (模拟器)"
echo "3. 全部架构 (通用 APK)"
echo ""

read -p "请选择目标架构 (1/2/3): " TARGET_CHOICE

case $TARGET_CHOICE in
    1)
        TARGET="aarch64-linux-android"
        echo "选择：aarch64"
        ;;
    2)
        TARGET="x86_64-linux-android"
        echo "选择：x86_64"
        ;;
    3)
        TARGET="all"
        echo "选择：全部架构"
        ;;
    *)
        echo -e "${RED}❌ 无效选择${NC}"
        exit 1
        ;;
esac

# 开始构建
echo ""
echo "🏗️  开始构建..."
echo "============="

cd "$(dirname "$0")"

# 安装前端依赖
echo "📦 安装前端依赖..."
npm install --prefer-offline

# 构建命令
if [ "$BUILD_CHOICE" = "1" ]; then
    BUILD_TYPE="debug"
    BUILD_FLAG="--debug"
else
    BUILD_TYPE="release"
    BUILD_FLAG="--release"
fi

echo ""
echo "🚀 执行构建命令..."
echo "构建类型：$BUILD_TYPE"
echo "目标架构：$TARGET"
echo ""

if [ "$ANDROID_SDK_FOUND" = false ]; then
    echo -e "${RED}❌ Android SDK 未找到，无法继续构建${NC}"
    echo ""
    echo "请按照以下步骤安装 Android SDK:"
    echo "1. 下载 Android Studio: https://developer.android.com/studio"
    echo "2. 或参考 ANDROID_BUILD.md 文档"
    exit 1
fi

# 执行构建
if [ "$TARGET" = "all" ]; then
    cargo tauri android build $BUILD_FLAG
else
    cargo tauri android build $BUILD_FLAG --target $TARGET
fi

# 检查结果
if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ 构建成功！${NC}"
    echo ""
    echo "📦 APK 文件位置:"
    echo "   src-tauri/target/$TARGET/$BUILD_TYPE/pure-live-rust.apk"
    echo ""
    
    # 询问是否安装
    if command -v adb &> /dev/null && [ "$DEVICES" -gt 0 ]; then
        read -p "是否立即安装到设备？(y/n): " INSTALL_CHOICE
        if [ "$INSTALL_CHOICE" = "y" ]; then
            echo "📲 正在安装..."
            APK_PATH="src-tauri/target/$TARGET/$BUILD_TYPE/pure-live-rust.apk"
            adb install -r "$APK_PATH"
            
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}✅ 安装成功！${NC}"
            else
                echo -e "${RED}❌ 安装失败${NC}"
            fi
        fi
    fi
else
    echo ""
    echo -e "${RED}❌ 构建失败${NC}"
    echo ""
    echo "请检查错误信息并参考 ANDROID_BUILD.md 文档"
    exit 1
fi

echo ""
echo "🎉 构建完成！"
