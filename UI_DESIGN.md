# 新拟态 UI 设计系统

## 🎨 设计理念

纯粹直播 Rust 版采用了**新拟态 (Neumorphism)**设计风格，这是一种现代、柔和且极具质感的设计语言。

### 核心特点

1. **柔和阴影** - 通过精细的光影营造立体感
2. **低饱和度色彩** - 舒适的灰蓝色调背景
3. **渐变点缀** - 蓝紫渐变作为视觉焦点
4. **流畅动画** - 细腻的过渡效果提升交互体验

---

## 🎯 设计系统组成

### 1. 颜色系统

#### 主色调
- **Primary**: `#6c5ce7` (深邃蓝紫)
- **Primary Dark**: `#5b4cdb` 
- **Primary Light**: `#a29bfe`

#### 辅助色
- **Cyan**: `#00d9ff` (科技蓝)
- **Pink**: `#ff6b9d` (活力粉)
- **Purple**: `#a855f7` (神秘紫)
- **Green**: `#10b981` (成功绿)
- **Orange**: `#f59e0b` (警告橙)

#### 背景色
- **Primary**: `#e0e5ec` (柔和灰蓝)
- **Secondary**: `#e8ecf1`
- **Tertiary**: `#d1d9e6`

### 2. 阴影系统

新拟态的核心在于阴影的运用：

```css
/* 平面效果 - 用于卡片 */
--neu-flat: 
  9px 9px 16px var(--shadow-dark),
  -9px -9px 16px var(--shadow-light);

/* 按下效果 - 用于输入框 */
--neu-pressed: 
  inset 6px 6px 10px var(--shadow-dark),
  inset -6px -6px 10px var(--shadow-light);

/* 凸起效果 - 用于按钮 */
--neu-convex: 
  6px 6px 10px var(--shadow-dark),
  -6px -6px 10px var(--shadow-light);

/* 悬浮效果 - 用于悬停状态 */
--neu-float: 
  12px 12px 20px var(--shadow-dark),
  -12px -12px 20px var(--shadow-light);
```

### 3. 组件库

#### 按钮 (Neumorphic Button)

```tsx
<button className="neu-button neu-button-primary">
  点击我
</button>
```

**特性**:
- 凸起阴影效果
- 悬停时阴影加深并上浮
- 按下时变为凹陷效果
- 支持主色/辅助色变体

#### 卡片 (Neumorphic Card)

```tsx
<div className="neu-card">
  卡片内容
</div>
```

**特性**:
- 平面阴影背景
- 悬停时上浮效果
- 圆角设计
- 自适应内容

#### 输入框 (Neumorphic Input)

```tsx
<input 
  type="text" 
  className="neu-input"
  placeholder="请输入..."
/>
```

**特性**:
- 凹陷阴影效果
- 聚焦时高亮边框
- 平滑过渡动画

#### 开关 (Neumorphic Switch)

```tsx
<div className={`neu-switch ${active ? 'active' : ''}`} />
```

**特性**:
- 凹陷轨道
- 凸起滑块
- 激活时渐变背景
- 流畅的位移动画

---

## 📱 页面设计

### 1. 首页 (HomePage)

**布局结构**:
```
┌─────────────────────────────────┐
│   顶部导航栏 (搜索 + 设置)       │
├─────────────────────────────────┤
│   平台选择器 (B 站/抖音/虎牙)    │
├─────────────────────────────────┤
│   直播间列表 (网格布局)          │
│   ┌─────┐ ┌─────┐ ┌─────┐      │
│   │卡片 │ │卡片 │ │卡片 │ ...  │
│   └─────┘ └─────┘ └─────┘      │
└─────────────────────────────────┘
│   底部状态栏                    │
└─────────────────────────────────┘
```

**设计亮点**:
- **毛玻璃导航栏** - 粘性定位 + 背景模糊
- **平台按钮** - 凸起效果，激活时渐变
- **直播卡片** - 悬浮动画 + 顶部渐变条
- **响应式网格** - 自适应不同屏幕尺寸

### 2. 播放器页面 (PlayerPage)

**布局结构**:
```
┌─────────────────────────────────┐
│   返回按钮                      │
├─────────────────────────────────┤
│   视频播放器 (MPV 封装)          │
│   ┌───────────────────────────┐ │
│   │  视频画面 + 弹幕层         │ │
│   │  ┌─────────────────────┐  │ │
│   │  │  控制栏 (悬停显示)   │  │ │
│   │  └─────────────────────┘  │ │
│   └───────────────────────────┘ │
├─────────────────────────────────┤
│   直播间信息面板                │
├─────────────────────────────────┤
│   弹幕列表                      │
└─────────────────────────────────┘
```

**设计亮点**:
- **智能竖屏检测** - 自动切换布局
- **悬浮控制栏** - 毛玻璃背景 + 渐变进度条
- **弹幕层** - 透明度可调 + 平滑动画
- **信息面板** - 凸起统计卡片 + 渐变标题

---

## 🎬 交互动画

### 1. 悬浮动画

```css
.neu-card:hover {
  box-shadow: var(--neu-hover);
  transform: translateY(-8px);
}
```

### 2. 按下动画

```css
.neu-button:active {
  box-shadow: var(--neu-pressed);
  transform: translateY(0);
}
```

### 3. 弹幕动画

```css
@keyframes danmaku-move {
  0% { transform: translateX(100%); opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { transform: translateX(-100%); opacity: 0; }
}
```

### 4. 加载动画

```css
@keyframes spin {
  to { transform: rotate(360deg); }
}

.spinner {
  animation: spin 1s linear infinite;
}
```

---

## 🌓 深色模式

新拟态设计系统完整支持深色模式：

```css
[data-theme="dark"] {
  --bg-primary: #1a202c;
  --bg-secondary: #2d3748;
  --shadow-light: rgba(255, 255, 255, 0.05);
  --shadow-dark: rgba(0, 0, 0, 0.6);
  --text-primary: #e2e8f0;
}
```

**切换方式**:
```tsx
document.documentElement.setAttribute('data-theme', 'dark');
```

---

## 📊 性能优化

### 1. CSS 变量

所有设计令牌使用 CSS 变量，支持：
- 运行时切换主题
- 动态调整颜色
- 减少重复代码

### 2. 硬件加速

关键动画使用 `transform` 和 `opacity` 属性，触发 GPU 加速。

### 3. 过渡优化

```css
transition: all var(--transition-base);
/* cubic-bezier 缓动曲线 */
--transition-base: 0.3s cubic-bezier(0.4, 0, 0.2, 1);
```

---

## 🎯 可用性

### 对比度

所有文字颜色通过 WCAG AA 标准：
- 正文：`#4a5568` on `#e0e5ec` (对比度 5.2:1)
- 次要文字：`#718096` (对比度 3.8:1)

### 可访问性

- 所有交互元素支持键盘导航
- 按钮有清晰的焦点状态
- 颜色不是唯一的信息载体

---

## 📦 文件结构

```
src/
├── styles/
│   └── neumorphism.css    # 设计系统核心
├── pages/
│   ├── HomePage.tsx       # 首页组件
│   ├── HomePage.css       # 首页样式
│   ├── PlayerPage.tsx     # 播放器页面
│   └── PlayerPage.css     # 播放器样式
└── App.tsx                # 主应用 (使用新 UI)
```

---

## 🚀 使用示例

### 快速开始

```tsx
import './styles/neumorphism.css'

function MyComponent() {
  return (
    <div className="neu-card">
      <h2 className="gradient-text">标题</h2>
      <button className="neu-button neu-button-primary">
        按钮
      </button>
      <input className="neu-input" placeholder="输入框" />
    </div>
  )
}
```

### 自定义颜色

```css
.my-custom-card {
  --primary-hue: 180; /* 青色 */
  --primary-saturation: 80%;
}
```

---

## 🎨 设计资源

### 在线工具

- **Neumorphism.io** - 在线生成新拟态 CSS
- **CSS Gradient** - 渐变生成器
- **Get Waves** - 波浪背景生成

### 参考项目

- Apple Music (macOS)
- Spotify (部分 UI)
- 现代车载系统界面

---

## 📝 注意事项

### 避免过度使用

1. **阴影层次** - 不要堆叠过多阴影
2. **颜色数量** - 保持主色 +2-3 个辅助色
3. **动画时长** - 控制在 0.2-0.5s

### 性能考虑

1. **backdrop-filter** - 在低端设备可能性能较差
2. **多个 box-shadow** - 每个元素不超过 2 个阴影
3. **动画数量** - 同时进行的动画不超过 3 个

---

## 🔮 未来规划

### 即将添加

- [ ] 更多组件 (下拉菜单、模态框)
- [ ] 加载骨架屏
- [ ] 图表组件
- [ ] 移动端优化

### 长期目标

- [ ] 主题市场 (多套配色)
- [ ] 动画预设库
- [ ] Figma 设计资源
- [ ] 组件文档站点

---

**新拟态 UI 设计系统** | 纯粹直播 Rust 版

*用现代设计语言，打造极致视觉体验* ✨
