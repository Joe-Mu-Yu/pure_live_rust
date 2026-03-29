//! 斗鱼弹幕处理（待实现）

use crate::error::Result;

pub async fn start_danmaku(_room_id: String) -> Result<()> {
    todo!("实现斗鱼弹幕接收")
}

pub async fn stop_danmaku(_room_id: &str) -> Result<()> {
    Ok(())
}
