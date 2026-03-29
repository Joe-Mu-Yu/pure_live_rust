import { useState, useRef } from 'react'
import '../styles/neumorphism.css'
import './PlayerPage.css'

interface PlayerPageProps {
  roomInfo?: {
    title: string
    anchor: string
    viewers: number
    site: string
  }
}

function PlayerPage({ roomInfo }: PlayerPageProps) {
  const [isPlaying, setIsPlaying] = useState(false)
  const [isMuted, setIsMuted] = useState(false)
  const [volume, setVolume] = useState(80)
  const [showDanmaku, setShowDanmaku] = useState(true)
  const [danmakuOpacity, setDanmakuOpacity] = useState(100)
  const [isPortrait, setIsPortrait] = useState(false)
  const [isFullscreen, setIsFullscreen] = useState(false)
  const videoRef = useRef<HTMLDivElement>(null)

  // 模拟竖屏检测
  const checkOrientation = () => {
    // 实际项目中会调用 Rust 后端 API
    const isVertical = Math.random() > 0.5
    setIsPortrait(isVertical)
  }

  // 切换播放/暂停
  const togglePlay = () => {
    setIsPlaying(!isPlaying)
  }

  // 切换全屏
  const toggleFullscreen = () => {
    setIsFullscreen(!isFullscreen)
  }

  // 切换弹幕
  const toggleDanmaku = () => {
    setShowDanmaku(!showDanmaku)
  }

  return (
    <div className={`player-page ${isFullscreen ? 'fullscreen' : ''}`}>
      {/* 返回按钮 */}
      <button className="back-button neu-button" onClick={() => window.history.back()}>
        ← 返回
      </button>

      {/* 视频播放区 */}
      <div className={`video-container ${isPortrait ? 'portrait' : ''} ${isFullscreen ? 'fullscreen' : ''}`}>
        <div 
          ref={videoRef}
          className="video-wrapper neu-pressed"
          onClick={togglePlay}
        >
          {/* 视频占位符 */}
          <div className="video-placeholder">
            {!isPlaying ? (
              <>
                <div className="play-icon neu-float" onClick={togglePlay}>
                  ▶️
                </div>
                <p className="placeholder-text">点击播放</p>
              </>
            ) : (
              <div className="loading-spinner">
                <div className="spinner"></div>
                <p>正在加载...</p>
              </div>
            )}
          </div>

          {/* 弹幕层 */}
          {showDanmaku && (
            <div 
              className="danmaku-layer"
              style={{ opacity: danmakuOpacity / 100 }}
            >
              <div className="danmaku-item" style={{ top: '10%', left: '0%' }}>这是一条测试弹幕 ✨</div>
              <div className="danmaku-item" style={{ top: '30%', left: '20%' }}>新拟态 UI 真好看！🎨</div>
              <div className="danmaku-item" style={{ top: '50%', left: '10%' }}>Rust 性能很强 🚀</div>
              <div className="danmaku-item" style={{ top: '70%', left: '30%' }}>竖屏检测很智能 📱</div>
            </div>
          )}
        </div>

        {/* 视频控制栏 */}
        <div className="video-controls neu-flat">
          {/* 进度条（直播无进度条，显示时间） */}
          <div className="progress-bar-container">
            <div className="progress-bar">
              <div className="progress-live">
                <span>LIVE</span>
              </div>
            </div>
          </div>

          {/* 控制按钮 */}
          <div className="control-buttons">
            <div className="left-controls">
              <button 
                className="neu-button control-button"
                onClick={togglePlay}
              >
                {isPlaying ? '⏸️' : '▶️'}
              </button>
              
              <button 
                className={`neu-button control-button ${isMuted ? 'muted' : ''}`}
                onClick={() => setIsMuted(!isMuted)}
              >
                {isMuted ? '🔇' : '🔊'}
              </button>
              
              <input
                type="range"
                min="0"
                max="100"
                value={isMuted ? 0 : volume}
                onChange={(e) => setVolume(Number(e.target.value))}
                className="neu-slider volume-slider"
              />
            </div>

            <div className="right-controls">
              <button 
                className={`neu-button control-button ${showDanmaku ? 'active' : ''}`}
                onClick={toggleDanmaku}
              >
                💬
              </button>
              
              <div className="danmaku-settings">
                <label>透明度</label>
                <input
                  type="range"
                  min="0"
                  max="100"
                  value={danmakuOpacity}
                  onChange={(e) => setDanmakuOpacity(Number(e.target.value))}
                  className="neu-slider"
                />
              </div>
              
              <button 
                className="neu-button control-button"
                onClick={checkOrientation}
              >
                📱
              </button>
              
              <button 
                className="neu-button control-button"
                onClick={toggleFullscreen}
              >
                {isFullscreen ? '❐' : '⛶'}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* 直播间信息 */}
      <div className="room-info-panel neu-flat">
        <div className="info-header">
          <div className="room-meta">
            <h1 className="room-title gradient-text">
              {roomInfo?.title || '测试直播间'}
            </h1>
            <p className="anchor-name">👤 {roomInfo?.anchor || '测试主播'}</p>
          </div>
          <div className="room-stats">
            <div className="stat-item neu-convex">
              <span className="stat-icon">👁️</span>
              <span className="stat-value">{roomInfo?.viewers.toLocaleString() || 12345}</span>
            </div>
            <div className="stat-item neu-convex">
              <span className="stat-icon">❤️</span>
              <span className="stat-value">9999+</span>
            </div>
          </div>
        </div>

        {/* 操作按钮 */}
        <div className="room-actions">
          <button className="neu-button neu-button-primary">
            ⭐ 收藏
          </button>
          <button className="neu-button">
            📤 分享
          </button>
          <button className="neu-button">
            🎁 打赏
          </button>
        </div>
      </div>

      {/* 弹幕列表 */}
      <div className="danmaku-panel neu-flat">
        <div className="panel-header">
          <h3>💬 弹幕列表</h3>
          <button className="neu-button">清空</button>
        </div>
        <div className="danmaku-list">
          {[1, 2, 3, 4, 5, 6].map((i) => (
            <div key={i} className="danmaku-message">
              <span className="username">用户{i}</span>
              <span className="message">这是一条测试弹幕 {i}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

export default PlayerPage
