# Android APK 构建指南

## 📱 环境要求

### 必需软件

1. **Android Studio** (推荐)
   - 下载地址：https://developer.android.com/studio
   - 或使用命令行工具

2. **Android SDK**
   - API Level: 24+ (Android 7.0)
   - Target SDK: 34 (Android 14)
   - Build Tools: 34.0.0+

3. **Java Development Kit (JDK)**
   - 版本：17 或更高
   - 下载地址：https://www.oracle.com/java/technologies/downloads/

4. **Rust + Cargo**
   - 版本：1.70+
   - 已安装 ✅

5. **Node.js**
   - 版本：18+
   - 已安装 ✅

---

## 🔧 环境配置

### 方法 1: 使用 Android Studio (推荐)

#### 步骤 1: 安装 Android Studio

```bash
# macOS (使用 Homebrew)
brew install --cask android-studio
```

#### 步骤 2: 安装 SDK 和构建工具

打开 Android Studio，然后：
1. 打开 SDK Manager
2. 安装以下组件：
   - Android SDK Platform 34
   - Android SDK Build-Tools 34.0.0
   - Android SDK Command-line Tools
   - Android Emulator (可选)

#### 步骤 3: 配置环境变量

添加到 `~/.zshrc` 或 `~/.bashrc`：

```bash
export ANDROID_HOME=$HOME/Library/Android/sdk
export ANDROID_SDK_ROOT=$HOME/Library/Android/sdk
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
```

然后执行：
```bash
source ~/.zshrc
```

---

### 方法 2: 命令行安装 (高级)

#### 步骤 1: 安装 SDK 命令行工具

```bash
# macOS
brew install android-commandlinetools
```

#### 步骤 2: 接受许可证并安装组件

```bash
# 创建 SDK 目录
mkdir -p $HOME/Library/Android/sdk

# 设置环境变量
export ANDROID_HOME=$HOME/Library/Android/sdk
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin

# 安装必要的 SDK 组件
sdkmanager "platform-tools"
sdkmanager "platforms;android-34"
sdkmanager "build-tools;34.0.0"

# 接受所有许可证
sdkmanager --licenses
```

---

## 🦀 Rust Android 目标配置

### 安装 Android 编译目标

```bash
# 添加 aarch64 (ARM64) 目标 - 现代 Android 设备
rustup target add aarch64-linux-android

# 添加 x86_64 目标 - 模拟器
rustup target add x86_64-linux-android

# 可选：添加 ARMv7 目标 - 旧设备
rustup target add armv7-linux-androideabi
```

### 安装 cargo-ndk (可选但推荐)

```bash
cargo install cargo-ndk
```

---

## 🏗️ 构建 APK

### 前提条件

确保已安装：
- ✅ Android SDK
- ✅ JDK 17+
- ✅ Rust Android 目标

### 步骤 1: 配置 Tauri

在 `src-tauri/tauri.conf.json` 中已配置 Android：

```json
{
  "bundle": {
    "android": {
      "minSdkVersion": 24,
      "targetSdkVersion": 34
    }
  }
}
```

### 步骤 2: 构建 APK

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 方法 1: 使用 npm 脚本
npm run tauri android build

# 方法 2: 直接使用 Tauri CLI
cargo tauri android build

# 方法 3: 构建 Debug 版本 (更快)
cargo tauri android build --debug
```

### 步骤 3: 构建特定 ABI

```bash
# 仅构建 ARM64 (推荐用于真机)
cargo tauri android build --target aarch64-linux-android

# 仅构建 x86_64 (用于模拟器)
cargo tauri android build --target x86_64-linux-android

# 构建所有 ABI (通用 APK)
cargo tauri android build --target all
```

---

## 📲 安装和测试

### 方法 1: 通过 ADB 安装

```bash
# 连接设备或启动模拟器
adb devices

# 安装 APK
adb install src-tauri/target/aarch64-linux-android/release/pure-live-rust.apk

# 或者使用 debug 版本
adb install src-tauri/target/aarch64-linux-android/debug/pure-live-rust.apk
```

### 方法 2: 直接运行到设备

```bash
# 连接设备后直接运行
cargo tauri android dev

# 或指定设备
cargo tauri android dev --device <device-id>
```

### 方法 3: 使用 Android Studio

1. 打开 `src-tauri/gen/android`
2. 用 Android Studio 打开项目
3. 点击 Run 按钮

---

## 🐛 常见问题

### 问题 1: 找不到 Android SDK

**错误**: `Android SDK not found`

**解决**:
```bash
# 确认 SDK 路径
echo $ANDROID_HOME
echo $ANDROID_SDK_ROOT

# 如果为空，添加到 ~/.zshrc
export ANDROID_HOME=$HOME/Library/Android/sdk
export ANDROID_SDK_ROOT=$HOME/Library/Android/sdk
```

### 问题 2: Rust 目标下载失败

**错误**: `component download failed for rust-std-aarch64-linux-android`

**解决**:
```bash
# 使用官方源
export RUSTUP_DIST_SERVER=https://static.rust-lang.org
rustup target add aarch64-linux-android
```

### 问题 3: Java 版本不兼容

**错误**: `Unsupported class file major version`

**解决**:
```bash
# 检查 Java 版本
java -version

# 需要 Java 17 或更高
# 如果版本过低，从 Oracle 下载最新版
```

### 问题 4: 构建时间过长

**优化**:
```bash
# 仅构建单个 ABI (更快)
cargo tauri android build --target aarch64-linux-android

# 使用 Debug 模式 (开发时)
cargo tauri android build --debug

# 启用增量编译
export CARGO_INCREMENTAL=1
```

---

## 📦 APK 输出位置

构建完成后，APK 文件位于：

```
src-tauri/target/aarch64-linux-android/release/
├── pure-live-rust.apk        # Release 版本 (优化后)
└── pure-live-rust-debug.apk  # Debug 版本 (开发用)
```

---

## 🚀 快速开始 (最简单的方式)

### 使用 Docker (无需安装 Android SDK)

```bash
# 1. 拉取 Tauri Android 构建镜像
docker pull ghcr.io/tauri-apps/tauri-android:latest

# 2. 进入项目目录
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 3. 使用 Docker 构建
docker run --rm -v $(pwd):/project -w /project \
  ghcr.io/tauri-apps/tauri-android:latest \
  cargo tauri android build
```

### 使用 GitHub Actions (云端构建)

创建 `.github/workflows/android-build.yml`:

```yaml
name: Build Android APK

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: aarch64-linux-android
      
      - name: Install Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20
      
      - name: Install dependencies
        run: npm install
      
      - name: Build APK
        run: cargo tauri android build
      
      - name: Upload APK
        uses: actions/upload-artifact@v4
        with:
          name: pure-live-rust-apk
          path: src-tauri/target/*/android/release/*.apk
```

---

## 📊 构建时间预估

| 设备 | 首次构建 | 增量构建 |
|------|---------|---------|
| M1/M2 Mac | ~5-8 分钟 | ~1-2 分钟 |
| Intel Mac | ~8-12 分钟 | ~2-4 分钟 |
| Docker | ~10-15 分钟 | ~3-5 分钟 |

---

## 🎯 性能优化建议

### 1. 使用 ProGuard 代码混淆

在 `src-tauri/gen/android/app/build.gradle` 中添加：

```gradle
android {
    buildTypes {
        release {
            minifyEnabled true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
}
```

### 2. 分离 ABI 构建

为不同设备构建不同 APK：

```bash
# ARM64 (现代设备)
cargo tauri android build --target aarch64-linux-android

# x86_64 (模拟器)
cargo tauri android build --target x86_64-linux-android
```

### 3. 启用 WebAssembly 优化

在 `Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = 'z'  # 优化体积
lto = true
codegen-units = 1
```

---

## 📱 测试设备推荐

### 真机测试
- Pixel 系列 (原生 Android)
- Samsung Galaxy 系列
- OnePlus 系列

### 模拟器
- Android Studio Emulator (推荐)
- Genymotion
- BlueStacks (游戏场景)

---

## 🔍 调试技巧

### 查看日志

```bash
# 查看应用日志
adb logcat | grep pure_live_rust

# 查看崩溃日志
adb logcat AndroidRuntime:E

# 清除日志
adb logcat -c
```

### 性能分析

```bash
# 查看 CPU 使用率
adb shell top | grep pure_live

# 查看内存使用
adb shell dumpsys meminfo com.morgan.purelive
```

---

## 📝 检查清单

构建前确认：
- [ ] Android SDK 已安装
- [ ] JDK 17+ 已安装
- [ ] Rust Android 目标已添加
- [ ] 环境变量已配置
- [ ] 设备已连接或模拟器已启动

构建后检查：
- [ ] APK 文件已生成
- [ ] 应用可正常安装
- [ ] 应用可正常启动
- [ ] 基本功能可正常使用
- [ ] 竖屏检测功能正常
- [ ] 弹幕显示正常

---

## 🆘 获取帮助

如果遇到问题：

1. **查看错误日志**
   ```bash
   cargo tauri android build --verbose
   ```

2. **检查 Tauri 文档**
   - https://tauri.app/v1/guides/building/android

3. **查看社区**
   - GitHub Issues
   - Discord 社区
   - Stack Overflow

---

**祝您构建顺利！** 🎉

如有问题，请参考本文档或查看错误日志。
