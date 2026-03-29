# Android APK 构建 - 快速指南

## 🎯 三种构建方式

根据您的情况选择最适合的方式：

---

## 方式 1: 本地完整构建 (推荐有 Android Studio 用户)

### 前提条件
- ✅ 已安装 Android Studio
- ✅ 已安装 JDK 17+
- ✅ 已配置环境变量

### 快速构建

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 运行自动化构建脚本
./build-android.sh

# 或手动构建
npm install
cargo tauri android build --release
```

### APK 位置
```
src-tauri/target/aarch64-linux-android/release/pure-live-rust.apk
```

---

## 方式 2: GitHub Actions 云端构建 (最简单 ⭐)

### 优势
- ✅ 无需安装 Android SDK
- ✅ 无需配置环境
- ✅ 自动构建多个架构
- ✅ 生成可下载链接

### 步骤

#### 1. 推送到 GitHub

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 初始化 Git
git init
git add .
git commit -m "Build Android APK"

# 创建 GitHub 仓库并推送
git remote add origin https://github.com/YOUR_USERNAME/pure_live_rust.git
git push -u origin main
```

#### 2. 触发构建

- **自动触发**: 推送代码或创建 Pull Request
- **手动触发**: 访问 Actions -> Build Android APK -> Run workflow

#### 3. 下载 APK

1. 访问仓库的 **Actions** 标签
2. 选择最新的构建
3. 在 **Artifacts** 部分下载 APK

---

## 方式 3: Docker 构建 (无需 Android SDK)

### 前提条件
- ✅ 已安装 Docker Desktop

### 步骤

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 使用 Docker 构建
docker run --rm -it \
  -v $(pwd):/project \
  -w /project \
  ghcr.io/tauri-apps/tauri-android:latest \
  cargo tauri android build --release
```

---

## 📱 安装测试

### 方法 1: ADB 安装

```bash
# 连接设备
adb devices

# 安装 APK
adb install src-tauri/target/aarch64-linux-android/release/pure-live-rust.apk
```

### 方法 2: 直接传输

1. 将 APK 文件发送到手机
2. 在手机上点击安装
3. 允许"未知来源"应用

---

## 🐛 遇到问题？

### 问题 1: 没有 Android SDK

**解决**: 使用方式 2 (GitHub Actions) 或方式 3 (Docker)

### 问题 2: Rust 目标下载失败

```bash
# 使用官方源
export RUSTUP_DIST_SERVER=https://static.rust-lang.org
rustup target add aarch64-linux-android
```

### 问题 3: 构建时间过长

```bash
# 仅构建 ARM64 (更快)
cargo tauri android build --target aarch64-linux-android

# 或使用 Debug 模式
cargo tauri android build --debug
```

---

## 📊 构建时间对比

| 方式 | 首次构建 | 增量构建 | 难度 |
|------|---------|---------|------|
| 本地构建 | 5-8 分钟 | 1-2 分钟 | ⭐⭐⭐ |
| GitHub Actions | 15-20 分钟 | - | ⭐ |
| Docker | 10-15 分钟 | 3-5 分钟 | ⭐⭐ |

---

## 🎯 推荐流程

### 快速测试 (5 分钟)
```bash
# Web 版本
npm run dev
```

### 桌面版测试 (10 分钟)
```bash
# macOS 版本
npm run tauri dev
```

### Android 测试 (20 分钟)
```bash
# 推送到 GitHub，使用 Actions 构建
git push
# 然后等待 Actions 完成
```

---

## 📁 相关文件

- [`ANDROID_BUILD.md`](./ANDROID_BUILD.md) - 详细构建文档
- [`build-android.sh`](./build-android.sh) - 自动化构建脚本
- [`ALTERNATIVE_TESTING.md`](./ALTERNATIVE_TESTING.md) - 替代测试方案
- [`.github/workflows/android-build.yml`](./.github/workflows/android-build.yml) - GitHub Actions 配置

---

## ✅ 检查清单

构建前：
- [ ] 已阅读构建文档
- [ ] 已选择构建方式
- [ ] 已准备必要环境

构建后：
- [ ] APK 文件已生成
- [ ] 应用可正常安装
- [ ] 基本功能测试通过
- [ ] UI 显示正常
- [ ] 性能表现良好

---

**选择适合您的方式，开始构建吧！** 🚀

如需帮助，请查看详细文档或提交 Issue。
