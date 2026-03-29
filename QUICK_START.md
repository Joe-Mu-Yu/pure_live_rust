# 快速启动指南

## 🚀 5 分钟快速开始

### 前置要求

确保已安装:
- ✅ Rust 1.70+ (检查：`rustc --version`)
- ✅ Node.js 18+ (检查：`node --version`)
- ✅ Git

### 第一步：安装 MPV 播放器

**macOS**:
```bash
brew install mpv
```

**Windows**:
下载并安装：https://mpv.io/installation/

**Linux**:
```bash
# Ubuntu/Debian
sudo apt install mpv libmpv-dev

# Arch Linux
sudo pacman -S mpv
```

### 第二步：克隆项目

```bash
cd /Users/morgan/WorkSpace/03-项目
git clone https://github.com/liuchuancong/pure_live.git pure_live_original  # 原项目参考
# pure_live_rust 已经创建完成
```

### 第三步：安装依赖

```bash
cd pure_live_rust

# 安装 Node.js 依赖
npm install
```

### 第四步：运行开发服务器

```bash
# 启动 Tauri 开发模式
npm run tauri dev
```

首次运行会自动:
- 编译 Rust 后端 (~2-5 分钟)
- 启动 Vite 开发服务器
- 打开应用窗口

### 第五步：测试功能

应用启动后，可以测试:

1. **数据库功能** ✅
   - 收藏/取消收藏
   - 查看历史记录

2. **播放器竖屏检测** ✅
   - 播放竖屏直播流
   - 自动切换到竖屏模式

3. **B 站 API** 🟡
   - 获取房间信息 (需实现完整签名)

---

## 🔧 常见问题

### Q1: Rust 编译失败

**错误**: `error[E0658]: use of unstable library feature...`

**解决**: 更新 Rust 到最新版
```bash
rustup update
```

### Q2: MPV 找不到

**错误**: `PlayerError("无法创建 MPV 实例")`

**解决**: 
- macOS: `brew install mpv`
- Windows: 确保 mpv.dll 在 PATH 中

### Q3: npm install 失败

**错误**: `npm ERR! code ENOENT`

**解决**:
```bash
# 清理缓存
npm cache clean --force

# 删除 node_modules
rm -rf node_modules package-lock.json

# 重新安装
npm install
```

### Q4: Tauri 启动失败

**错误**: `could not find tauri.conf.json`

**解决**: 确保在 `pure_live_rust` 目录下运行

---

## 📝 开发技巧

### 调试 Rust 后端

```bash
# 启用详细日志
RUST_LOG=debug npm run tauri dev

# 查看日志输出
# 日志会显示在终端和开发者工具中
```

### 调试前端

```bash
# 打开开发者工具
# 在应用窗口右键 -> 检查

# 查看控制台日志
# Tauri 事件监听
listen('danmaku-message', (event) => {
  console.log('弹幕:', event.payload);
});
```

### 热重载

- **前端修改**: 自动热重载
- **Rust 修改**: 自动重新编译 (较慢)

---

## 🎯 下一步

### 测试 B 站功能

1. 完善 `sites/bilibili.rs` 中的签名算法
2. 测试真实直播间:
```typescript
const room = await invoke('get_room_info', {
  site: 'bilibili',
  roomId: '6'  // 测试房间 ID
});
```

### 开发前端 UI

参考 `ARCHITECTURE.md` 中的 UI 设计规范

### 添加新平台

参考 `sites/bilibili.rs` 的实现模板

---

## 📚 更多文档

- [README.md](./README.md) - 项目总览
- [ARCHITECTURE.md](./ARCHITECTURE.md) - 架构设计
- [PROGRESS.md](./PROGRESS.md) - 开发进度
- [DELIVERY_SUMMARY.md](./DELIVERY_SUMMARY.md) - 交付总结

---

**祝您开发愉快！** 🎉
