//! 网易 CC 直播间适配（待实现）

use crate::error::Result;
use crate::commands::{RoomInfo, StreamUrl};

pub async fn get_room_info(_room_id: &str) -> Result<RoomInfo> {
    todo!("实现 CC 房间信息获取")
}

pub async fn get_live_streams(_room_id: &str) -> Result<Vec<StreamUrl>> {
    todo!("实现 CC 直播流获取")
}
