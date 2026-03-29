//! 播放器模块（基于 MPV）

use crate::error::{Result, AppError};
use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

static PLAYER: once_cell::sync::Lazy<Arc<Mutex<Option<MpvPlayer>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

/// MPV 播放器封装
struct MpvPlayer {
    mpv: libmpv_sys::mpv_handle,
    current_orientation: VideoOrientation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoOrientation {
    Portrait,   // 竖屏 (height > width)
    Landscape,  // 横屏 (width > height)
    Square,     // 正方形
}

impl MpvPlayer {
    fn new() -> Result<Self> {
        let mpv = unsafe { libmpv_sys::mpv_create() };
        if mpv.is_null() {
            return Err(AppError::PlayerError("无法创建 MPV 实例".to_string()));
        }
        
        // 初始化 MPV
        unsafe {
            libmpv_sys::mpv_initialize(mpv);
            
            // 设置硬件解码
            let hwdec = b"auto\0".as_ptr() as *const _;
            libmpv_sys::mpv_set_option_string(mpv, b"hwdec\0".as_ptr() as *const _, hwdec);
            
            // 设置缓存
            let cache = b"yes\0".as_ptr() as *const _;
            libmpv_sys::mpv_set_option_string(mpv, b"cache\0".as_ptr() as *const _, cache);
        }
        
        Ok(MpvPlayer {
            mpv,
            current_orientation: VideoOrientation::Landscape,
        })
    }
    
    fn play(&mut self, url: &str, headers: Option<&HashMap<String, String>>) -> Result<()> {
        let url_c = std::ffi::CString::new(url).unwrap();
        let url_ptr = url_c.as_ptr();
        
        unsafe {
            // 设置播放 URL
            libmpv_sys::mpv_command_string(
                self.mpv,
                format!("loadfile {}\0", url).as_ptr() as *const _,
            );
            
            // 设置 headers
            if let Some(h) = headers {
                let headers_str = h.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let headers_c = std::ffi::CString::new(headers_str).unwrap();
                libmpv_sys::mpv_set_option_string(
                    self.mpv,
                    b"http-header-fields\0".as_ptr() as *const _,
                    headers_c.as_ptr(),
                );
            }
        }
        
        Ok(())
    }
    
    fn pause(&self) -> Result<()> {
        unsafe {
            libmpv_sys::mpv_set_property_string(
                self.mpv,
                b"pause\0".as_ptr() as *const _,
                b"yes\0".as_ptr() as *const _,
            );
        }
        Ok(())
    }
    
    fn resume(&self) -> Result<()> {
        unsafe {
            libmpv_sys::mpv_set_property_string(
                self.mpv,
                b"pause\0".as_ptr() as *const _,
                b"no\0".as_ptr() as *const _,
            );
        }
        Ok(())
    }
    
    fn stop(&self) -> Result<()> {
        unsafe {
            libmpv_sys::mpv_command_string(
                self.mpv,
                b"stop\0".as_ptr() as *const _,
            );
        }
        Ok(())
    }
    
    fn check_orientation(&mut self) -> VideoOrientation {
        unsafe {
            let mut width: i64 = 0;
            let mut height: i64 = 0;
            
            libmpv_sys::mpv_get_property(
                self.mpv,
                b"width\0".as_ptr() as *const _,
                libmpv_sys::mpv_format::MPV_FORMAT_INT64,
                &mut width as *mut _ as *mut _,
            );
            
            libmpv_sys::mpv_get_property(
                self.mpv,
                b"height\0".as_ptr() as *const _,
                libmpv_sys::mpv_format::MPV_FORMAT_INT64,
                &mut height as *mut _ as *mut _,
            );
            
            self.current_orientation = if height > width {
                VideoOrientation::Portrait
            } else if width > height {
                VideoOrientation::Landscape
            } else {
                VideoOrientation::Square
            };
            
            self.current_orientation
        }
    }
    
    fn set_portrait_mode(&self, enable: bool) {
        unsafe {
            if enable {
                // 设置为竖屏模式（旋转 90 度或调整宽高比）
                libmpv_sys::mpv_set_property_string(
                    self.mpv,
                    b"video-rotate\0".as_ptr() as *const _,
                    b"90\0".as_ptr() as *const _,
                );
            } else {
                libmpv_sys::mpv_set_property_string(
                    self.mpv,
                    b"video-rotate\0".as_ptr() as *const _,
                    b"0\0".as_ptr() as *const _,
                );
            }
        }
    }
}

/// 初始化播放器
pub fn init_player(_app_handle: &tauri::AppHandle) -> Result<()> {
    let player = MpvPlayer::new()?;
    *PLAYER.lock() = Some(player);
    tracing::info!("播放器初始化完成");
    Ok(())
}

/// 播放直播流
pub async fn play(url: &str, headers: Option<HashMap<String, String>>) -> Result<()> {
    let mut player_guard = PLAYER.lock();
    
    if let Some(player) = player_guard.as_mut() {
        player.play(url, headers.as_ref())?;
        
        // 检测视频方向并自动调整
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let orientation = player.check_orientation();
        
        if orientation == VideoOrientation::Portrait {
            player.set_portrait_mode(true);
            tracing::info!("检测到竖屏视频，已自动切换为竖屏模式");
        }
        
        Ok(())
    } else {
        Err(AppError::PlayerError("播放器未初始化".to_string()))
    }
}

/// 暂停播放
pub fn pause() -> Result<()> {
    let player_guard = PLAYER.lock();
    
    if let Some(player) = player_guard.as_ref() {
        player.pause()
    } else {
        Err(AppError::PlayerError("播放器未初始化".to_string()))
    }
}

/// 停止播放
pub fn stop() -> Result<()> {
    let player_guard = PLAYER.lock();
    
    if let Some(player) = player_guard.as_ref() {
        player.stop()
    } else {
        Err(AppError::PlayerError("播放器未初始化".to_string()))
    }
}

/// 获取视频方向
pub fn get_orientation() -> Result<String> {
    let player_guard = PLAYER.lock();
    
    if let Some(player) = player_guard.as_ref() {
        let orientation = match player.current_orientation {
            VideoOrientation::Portrait => "portrait",
            VideoOrientation::Landscape => "landscape",
            VideoOrientation::Square => "square",
        };
        Ok(orientation.to_string())
    } else {
        Err(AppError::PlayerError("播放器未初始化".to_string()))
    }
}
