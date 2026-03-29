# 纯粹直播 Rust 版 - 项目架构文档

## 项目概述

本项目是将原 Flutter 版本的 pure_live 直播聚合播放器重构为 Rust + Tauri + React 架构。

### 技术栈对比

| 层级 | 原技术栈 (Flutter) | 新技术栈 (Rust) |
|------|-------------------|----------------|
| **UI 框架** | Flutter 3.38.3 | React 19 + Vite |
| **后端逻辑** | Dart | Rust 2024 |
| **视频播放** | media_kit + MPV | libmpv-sys |
| **弹幕渲染** | Canvas | WebGL (计划) |
| **数据存储** | Hive | SQLite (rusqlite) |
| **跨平台** | Flutter 原生 | Tauri 2.x |

## 项目结构

```
pure_live_rust/
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs              # 应用入口
│   │   ├── lib.rs               # Tauri 应用配置
│   │   ├── commands.rs          # Tauri 命令处理
│   │   ├── error.rs             # 错误处理
│   │   ├── sites/               # 直播平台适配
│   │   │   ├── bilibili.rs      ✓ B 站适配 (已实现)
│   │   │   ├── douyin.rs        ⏳ 抖音适配
│   │   │   ├── huya.rs          ⏳ 虎牙适配
│   │   │   ├── douyu.rs         ⏳ 斗鱼适配
│   │   │   ├── kuaishou.rs      ⏳ 快手适配
│   │   │   └── cc.rs            ⏳ CC 适配
│   │   ├── danmaku/             # 弹幕处理
│   │   │   ├── bilibili.rs      ✓ B 站弹幕框架
│   │   │   ├── douyin.rs        ⏳ 抖音弹幕
│   │   │   ├── huya.rs          ⏳ 虎牙弹幕
│   │   │   └── douyu.rs         ⏳ 斗鱼弹幕
│   │   ├── player.rs            ✓ MPV 播放器封装 + 竖屏检测
│   │   ├── db.rs                ✓ SQLite 数据库
│   │   └── services/            # 外部服务
│   │       ├── webdav.rs        ⏳ WebDAV 同步
│   │       └── supabase.rs      ⏳ Supabase 认证
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src/                          # React 前端
│   ├── components/              # UI 组件
│   ├── pages/                   # 页面
│   ├── stores/                  # 状态管理
│   ├── assets/                  # 静态资源
│   └── App.tsx
├── package.json
└── README.md
```

## 核心功能实现状态

### ✅ 已完成

1. **项目基础架构**
   - Tauri 2.x + React + TypeScript 项目搭建
   - Rust 后端模块划分
   - 错误处理系统

2. **播放器模块** (`player.rs`)
   - MPV 播放器封装
   - **智能竖屏检测**：自动识别视频分辨率并切换竖屏模式
   - 硬件解码支持
   - 播放控制（播放/暂停/停止）

3. **数据库模块** (`db.rs`)
   - SQLite 数据库初始化
   - 收藏功能（增删查）
   - 历史记录（自动清理）

4. **B 站适配** (`sites/bilibili.rs`)
   - 房间信息获取
   - 直播流地址获取
   - 真实房间 ID 解析

5. **B 站弹幕框架** (`danmaku/bilibili.rs`)
   - WebSocket 连接
   - 弹幕协议封装

### ⏳ 待实现

1. **其他平台适配**
   - 抖音、虎牙、斗鱼、快手、CC 的 API 对接
   - 各平台弹幕协议解析

2. **前端 UI**
   - 新拟态风格设计系统
   - 直播列表页
   - 播放器界面
   - 弹幕渲染组件

3. **性能优化**
   - WebGL 弹幕渲染
   - 视频加载优化
   - 内存管理

4. **外部服务**
   - WebDAV 数据同步
   - Supabase 用户系统

## 竖屏播放优化方案

### 智能检测逻辑

```rust
// player.rs 中的核心逻辑
fn check_orientation(&mut self) -> VideoOrientation {
    let (width, height) = get_video_dimensions();
    
    self.current_orientation = if height > width {
        VideoOrientation::Portrait  // 竖屏
    } else if width > height {
        VideoOrientation::Landscape // 横屏
    } else {
        VideoOrientation::Square    // 正方形
    };
}
```

### 自动切换流程

1. 播放开始后 500ms 检测视频分辨率
2. 如果检测到竖屏（height > width）：
   - 自动设置 `video-rotate=90`
   - 调整播放器容器宽高比
   - 通知前端切换布局
3. 前端收到事件后：
   - 调整视频区域大小
   - 优化弹幕显示区域
   - 更新 UI 控件位置

## API 设计

### Tauri Commands

```typescript
// 前端调用示例
import { invoke } from '@tauri-apps/api/core';

// 获取房间信息
const roomInfo = await invoke('get_room_info', {
  site: 'bilibili',
  roomId: '123456'
});

// 获取直播流
const streams = await invoke('get_live_streams', {
  site: 'bilibili',
  roomId: '123456'
});

// 播放直播
await invoke('play_stream', {
  url: 'https://...',
  headers: { 'User-Agent': '...' }
});

// 开始弹幕
await invoke('start_danmaku', {
  site: 'bilibili',
  roomId: '123456'
});
```

## 性能目标

| 指标 | Flutter 版本 | Rust 目标 | 提升 |
|------|-------------|----------|------|
| 冷启动 | 2-3s | <1s | 60%+ |
| 内存占用 | 200-300MB | <100MB | 50%+ |
| 弹幕渲染 | 60fps | 120fps | 100%+ |
| 加载速度 | 1-2s | <500ms | 50%+ |

## 开发计划

### 阶段 1：基础架构 ✅

- [x] Tauri + React 项目初始化
- [x] Rust 后端模块设计
- [x] MPV 播放器集成
- [x] SQLite 数据库

### 阶段 2：核心功能迁移 🚧

- [x] B 站 API 适配
- [ ] 抖音 API 适配
- [ ] 虎牙 API 适配
- [ ] 斗鱼 API 适配
- [ ] 弹幕系统完善

### 阶段 3：UI 开发

- [ ] 新拟态设计系统
- [ ] 首页/列表页
- [ ] 播放器界面
- [ ] 弹幕渲染组件

### 阶段 4：优化与测试

- [ ] WebGL 弹幕渲染
- [ ] 性能基准测试
- [ ] 跨平台打包
- [ ] 用户文档

## 构建说明

### 开发模式

```bash
# 安装依赖
npm install

# 运行开发服务器
npm run tauri dev
```

### 发布构建

```bash
# Android
npm run tauri android build

# macOS
npm run tauri build --target universal-apple-darwin
```

## 注意事项

1. **MPV 依赖**：需要系统安装 mpv 库
2. **Protobuf**：弹幕解析需要编译 `.proto` 文件
3. **证书签名**：发布需要配置代码签名

## 参考资源

- [原项目 (Flutter)](../pure_live_original)
- [Tauri 文档](https://tauri.app/)
- [MPV 文档](https://mpv.io/manual/master/)
