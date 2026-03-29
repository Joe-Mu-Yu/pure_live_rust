# 替代测试方案

由于 Android SDK 需要额外安装，我们提供以下替代测试方案：

---

## 🌐 方案 1: Web 版本测试 (最简单)

### 使用条件
- ✅ 无需 Android SDK
- ✅ 无需 Java
- ✅ 5 分钟内可启动

### 步骤

#### 1. 构建 Web 版本

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 安装依赖
npm install

# 构建 Web 版本
npm run build
```

#### 2. 本地预览

```bash
# 使用 Vite 预览构建结果
npm run preview
```

访问 `http://localhost:4173` 查看应用

#### 3. 或使用开发服务器

```bash
# 启动开发服务器 (实时重载)
npm run dev
```

访问 `http://localhost:5173`

---

## 💻 方案 2: macOS 桌面版测试

### 使用条件
- ✅ 无需 Android SDK
- ✅ 已有 Rust 环境
- ✅ 10 分钟内可完成

### 步骤

#### 1. 构建 macOS 版本

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 构建 macOS 应用
npm run tauri build
```

#### 2. 运行开发版

```bash
# 开发模式运行
npm run tauri dev
```

应用会自动打开，可以测试所有功能

#### 3. 安装生产版本

构建完成后，应用位于：
```
src-tauri/target/release/bundle/macos/pure-live-rust.app
```

---

## 🐳 方案 3: Docker Android 构建 (云端)

### 使用条件
- ✅ 无需本地 Android SDK
- ✅ 需要 Docker
- ✅ 适合 CI/CD

### 步骤

#### 1. 使用 Docker 镜像

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 使用 Tauri Android 构建镜像
docker run --rm -it \
  -v $(pwd):/project \
  -w /project \
  ghcr.io/tauri-apps/tauri-android:latest \
  cargo tauri android build
```

#### 2. 获取 APK

构建完成后，APK 文件位于：
```
src-tauri/target/aarch64-linux-android/release/pure-live-rust.apk
```

---

## ☁️ 方案 4: GitHub Actions (推荐)

### 优势
- ✅ 完全自动化
- ✅ 无需本地环境
- ✅ 每次提交自动构建
- ✅ 生成可下载的 APK

### 步骤

#### 1. 创建 GitHub Action 工作流

创建 `.github/workflows/android-build.yml`:

```yaml
name: Build Android APK

on:
  push:
    tags:
      - 'v*'
  pull_request:
    branches: [ main ]

jobs:
  build-android:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      
      - name: Setup Java
        uses: actions/setup-java@v4
        with:
          distribution: 'temurin'
          java-version: '17'
      
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: aarch64-linux-android
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install dependencies
        run: |
          npm install
          rustup target add aarch64-linux-android
      
      - name: Build APK
        run: cargo tauri android build
      
      - name: Upload APK
        uses: actions/upload-artifact@v4
        with:
          name: pure-live-rust-android
          path: src-tauri/target/aarch64-linux-android/release/*.apk
```

#### 2. 推送代码到 GitHub

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 初始化 Git (如果还没有)
git init

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit: Pure Live Rust"

# 添加远程仓库 (替换为您的仓库)
git remote add origin https://github.com/your-username/pure_live_rust.git

# 推送到 GitHub
git push -u origin main
```

#### 3. 下载 APK

1. 访问 GitHub 仓库
2. 点击 "Actions" 标签
3. 选择最新的构建
4. 在 "Artifacts" 部分下载 APK

---

## 📱 方案 5: 使用在线 Android 测试平台

### 推荐平台

#### 1. Appetize.io
- 网址：https://appetize.io
- 免费额度：每月 100 分钟
- 支持：上传 APK 在线测试

#### 2. BrowserStack
- 网址：https://www.browserstack.com/app-automate
- 免费试用：14 天
- 支持：真实设备测试

#### 3. Sauce Labs
- 网址：https://saucelabs.com/
- 免费试用：14 天
- 支持：模拟器和真机

---

## 🎯 快速测试流程 (推荐)

### 第一步：Web 版本快速验证 (5 分钟)

```bash
# 1. 启动开发服务器
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust
npm run dev

# 2. 访问 http://localhost:5173
# 3. 测试 UI 和基本功能
```

### 第二步：macOS 桌面版测试 (10 分钟)

```bash
# 1. 运行开发版
npm run tauri dev

# 2. 测试完整功能
# - 新拟态 UI
# - 竖屏检测
# - 弹幕功能
```

### 第三步：GitHub Actions 构建 APK (30 分钟)

```bash
# 1. 创建 GitHub 仓库
# 2. 推送代码
# 3. 等待 Actions 完成
# 4. 下载 APK 到手机测试
```

---

## 📊 测试检查清单

### UI 测试
- [ ] 新拟态样式正确显示
- [ ] 渐变效果正常
- [ ] 按钮阴影效果
- [ ] 卡片悬浮动画
- [ ] 深色模式切换

### 功能测试
- [ ] 应用正常启动
- [ ] 页面导航流畅
- [ ] 按钮点击响应
- [ ] 输入框可输入
- [ ] 列表滚动流畅

### 性能测试
- [ ] 启动速度 <2 秒
- [ ] 页面切换流畅
- [ ] 无明显卡顿
- [ ] 内存占用合理

---

## 🔧 调试工具

### Web 版本

```bash
# 在浏览器中打开开发者工具
# Chrome: F12 或 Cmd+Option+I
# 查看控制台、网络、性能等
```

### macOS 版本

```bash
# 在 Tauri 应用中打开开发者工具
# 右键 -> 检查元素
# 或 Cmd+Option+I
```

### Android 版本

```bash
# 查看日志
adb logcat | grep pure_live

# 性能分析
adb shell dumpsys gfxinfo com.morgan.purelive
```

---

## 📝 测试报告模板

### 基本信息
- 测试日期：YYYY-MM-DD
- 测试版本：v0.1.0
- 测试平台：Web / macOS / Android
- 测试设备：(如适用)

### 测试结果

#### UI 显示
- ✅ 通过 / ❌ 失败
- 问题描述：...

#### 交互功能
- ✅ 通过 / ❌ 失败
- 问题描述：...

#### 性能表现
- ✅ 通过 / ❌ 失败
- 问题描述：...

### 截图/录屏
(附上关键功能的截图)

---

## 🆘 遇到问题？

### 常见问题解答

**Q: 构建时间太长？**
A: 使用 Debug 模式或仅构建单一架构

**Q: APK 无法安装？**
A: 检查 Android 版本是否 >= 7.0 (API 24)

**Q: 应用闪退？**
A: 查看 `adb logcat` 日志

**Q: UI 显示异常？**
A: 清除缓存并重新构建

---

## 📞 获取帮助

1. **查看文档**
   - [ANDROID_BUILD.md](./ANDROID_BUILD.md)
   - [README.md](./README.md)
   - [UI_DESIGN.md](./UI_DESIGN.md)

2. **提交 Issue**
   - GitHub Issues

3. **社区支持**
   - Discord
   - Stack Overflow

---

**选择最适合您的测试方案，开始测试吧！** 🚀
