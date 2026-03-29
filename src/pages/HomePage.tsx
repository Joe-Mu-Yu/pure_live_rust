import { useState, useEffect } from 'react'
import '../styles/neumorphism.css'
import './HomePage.css'

// 模拟直播数据
const mockLiveRooms = [
  { id: 1, site: 'bilibili', title: '测试直播间 1', anchor: '主播 A', viewers: 12345, avatar: '', isLiving: true },
  { id: 2, site: 'douyin', title: '抖音直播测试', anchor: '主播 B', viewers: 23456, avatar: '', isLiving: true },
  { id: 3, site: 'huya', title: '虎牙游戏直播', anchor: '主播 C', viewers: 34567, avatar: '', isLiving: true },
  { id: 4, site: 'douyu', title: '斗鱼娱乐直播', anchor: '主播 D', viewers: 45678, avatar: '', isLiving: false },
]

const platforms = [
  { id: 'all', name: '全部', icon: '📺' },
  { id: 'bilibili', name: 'B 站', icon: '📱' },
  { id: 'douyin', name: '抖音', icon: '🎵' },
  { id: 'huya', name: '虎牙', icon: '🐯' },
  { id: 'douyu', name: '斗鱼', icon: '🐟' },
]

function HomePage() {
  const [selectedPlatform, setSelectedPlatform] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [liveRooms, setLiveRooms] = useState(mockLiveRooms)
  const [isDarkMode, setIsDarkMode] = useState(false)

  // 切换深色模式
  const toggleDarkMode = () => {
    setIsDarkMode(!isDarkMode)
    document.documentElement.setAttribute('data-theme', !isDarkMode ? 'dark' : 'light')
  }

  // 过滤直播间
  const filteredRooms = liveRooms.filter(room => {
    const matchPlatform = selectedPlatform === 'all' || room.site === selectedPlatform
    const matchSearch = room.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                       room.anchor.toLowerCase().includes(searchQuery.toLowerCase())
    return matchPlatform && matchSearch
  })

  return (
    <div className="home-page">
      {/* 顶部导航栏 */}
      <header className="top-nav neu-flat">
        <div className="nav-content">
          <div className="logo">
            <span className="logo-icon">🎬</span>
            <h1 className="logo-text gradient-text">纯粹直播</h1>
          </div>
          
          <div className="search-bar">
            <input
              type="text"
              placeholder="搜索直播间或主播..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="neu-input search-input"
            />
          </div>
          
          <div className="nav-actions">
            <button 
              className={`neu-switch ${isDarkMode ? 'active' : ''}`}
              onClick={toggleDarkMode}
              title="切换深色模式"
            />
            <button className="neu-button neu-button-primary">
              <span>⚙️</span>
              设置
            </button>
          </div>
        </div>
      </header>

      {/* 主要内容区 */}
      <main className="main-content">
        {/* 平台选择器 */}
        <section className="platform-selector">
          <div className="neu-card platform-card">
            <h2 className="section-title">选择平台</h2>
            <div className="platform-buttons">
              {platforms.map(platform => (
                <button
                  key={platform.id}
                  className={`neu-button platform-button ${selectedPlatform === platform.id ? 'active' : ''}`}
                  onClick={() => setSelectedPlatform(platform.id)}
                >
                  <span className="platform-icon">{platform.icon}</span>
                  <span className="platform-name">{platform.name}</span>
                </button>
              ))}
            </div>
          </div>
        </section>

        {/* 直播列表 */}
        <section className="live-rooms-section">
          <div className="section-header">
            <h2 className="section-title gradient-text">
              {selectedPlatform === 'all' ? '全部直播间' : `${platforms.find(p => p.id === selectedPlatform)?.name}直播`}
            </h2>
            <span className="room-count">{filteredRooms.length} 个直播间</span>
          </div>
          
          <div className="rooms-grid">
            {filteredRooms.map(room => (
              <LiveRoomCard key={room.id} room={room} />
            ))}
          </div>
          
          {filteredRooms.length === 0 && (
            <div className="empty-state neu-flat">
              <div className="empty-icon">📭</div>
              <p className="empty-text">暂无直播间</p>
            </div>
          )}
        </section>
      </main>

      {/* 底部状态栏 */}
      <footer className="bottom-bar neu-flat">
        <div className="status-info">
          <span>✅ Rust 后端就绪</span>
          <span>🚀 高性能播放</span>
          <span>📱 智能竖屏</span>
        </div>
        <div className="version">v0.1.0</div>
      </footer>
    </div>
  )
}

// 直播间卡片组件
function LiveRoomCard({ room }) {
  const [isHovered, setIsHovered] = useState(false)

  const siteIcons: Record<string, string> = {
    bilibili: '📱',
    douyin: '🎵',
    huya: '🐯',
    douyu: '🐟',
  }

  return (
    <div 
      className={`live-room-card neu-card ${isHovered ? 'hovered' : ''}`}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div className="card-header">
        <div className="room-avatar neu-float">
          <span className="avatar-placeholder">{room.anchor[0]}</span>
        </div>
        <div className="site-badge neu-convex">
          {siteIcons[room.site] || '📺'}
        </div>
        {room.isLiving && (
          <div className="live-badge">
            <span className="live-dot"></span>
            直播中
          </div>
        )}
      </div>
      
      <div className="card-body">
        <h3 className="room-title">{room.title}</h3>
        <p className="anchor-name">{room.anchor}</p>
        <div className="room-stats">
          <span className="viewers">👁️ {room.viewers.toLocaleString()}</span>
        </div>
      </div>
      
      <div className="card-actions">
        <button className="neu-button neu-button-primary action-button">
          ▶️ 观看
        </button>
        <button className="neu-button action-button">
          ⭐ 收藏
        </button>
      </div>
    </div>
  )
}

export default HomePage
