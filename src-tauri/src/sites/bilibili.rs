//! B 站直播间适配

use crate::error::{Result, AppError};
use crate::commands::{RoomInfo, StreamUrl};
use reqwest::Client;
use serde_json::Value;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// 获取 B 站房间信息
pub async fn get_room_info(room_id: &str) -> Result<RoomInfo> {
    let client = Client::new();
    let url = format!("https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}", room_id);
    
    let response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    if json["code"].as_i64().unwrap_or(0) != 0 {
        return Err(AppError::NetworkError(json["msg"].as_str().unwrap_or("未知错误").to_string()));
    }
    
    let data = &json["data"];
    let room_info = RoomInfo {
        site: "bilibili".to_string(),
        room_id: room_id.to_string(),
        title: data["title"].as_str().unwrap_or("").to_string(),
        anchor_name: data["uname"].as_str().unwrap_or("").to_string(),
        avatar: data["user_cover"].as_str().unwrap_or("").to_string(),
        viewer_count: data["online"].as_u64().unwrap_or(0),
        is_living: data["live_status"].as_i64().unwrap_or(0) == 1,
        area_name: data["area_name"].as_str().unwrap_or("").to_string(),
    };
    
    Ok(room_info)
}

/// 获取 B 站直播流地址
pub async fn get_live_streams(room_id: &str) -> Result<Vec<StreamUrl>> {
    let client = Client::new();
    
    // 获取真实房间 ID
    let real_room_id = get_real_room_id(room_id).await?;
    
    // 获取播放 URL
    let url = format!("https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id={}&protocol=0,1&format=0,1,2&codec=0,1", real_room_id);
    
    let response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    if json["code"].as_i64().unwrap_or(0) != 0 {
        return Err(AppError::NetworkError(json["message"].as_str().unwrap_or("获取失败").to_string()));
    }
    
    let mut streams = Vec::new();
    
    if let Some(quality_list) = json["data"]["playurl_info"]["playurl"]["stream"][0]["format"][0]["codec"][0]["accept_qn"].as_array() {
        for qn in quality_list {
            if let Some(quality) = qn.as_u64() {
                let quality_name = match quality {
                    10000 => "原画",
                    400 => "蓝光 4M",
                    250 => "超清",
                    150 => "高清",
                    80 => "流畅",
                    _ => "未知",
                };
                
                // 构建播放 URL（需要签名）
                let play_url = format!("https://api.live.bilibili.com/xlive/web-room/v1/index/getRoomPlayInfo?room_id={}&qn={}&platform=web", real_room_id, quality);
                
                streams.push(StreamUrl {
                    quality: quality_name.to_string(),
                    url: play_url,
                    codec: "flv".to_string(),
                });
            }
        }
    }
    
    Ok(streams)
}

/// 获取真实房间 ID（处理短 ID）
async fn get_real_room_id(room_id: &str) -> Result<String> {
    let client = Client::new();
    let url = format!("https://api.live.bilibili.com/room/v1/Room/room_init?id={}", room_id);
    
    let response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    if let Some(real_id) = json["data"]["room_id"].as_str() {
        Ok(real_id.to_string())
    } else {
        Ok(room_id.to_string())
    }
}
