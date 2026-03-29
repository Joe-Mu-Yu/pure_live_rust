//! B 站弹幕处理

use crate::error::Result;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, TryStreamExt};
use url::Url;

/// 开始接收 B 站弹幕
pub async fn start_danmaku(room_id: String) -> Result<()> {
    // 获取弹幕 WebSocket 服务器地址
    let (host, room_id) = get_danmaku_server(&room_id).await?;
    
    // 构建 WebSocket URL
    let ws_url = format!("wss://{}/sub", host);
    let url = Url::parse(&ws_url).unwrap();
    
    // 连接 WebSocket
    let (ws_stream, _) = connect_async(url).await?;
    let (_, mut read) = ws_stream.split();
    
    // 发送加入房间消息
    let join_msg = build_join_message(&room_id);
    let _ = read.try_send(Message::Binary(join_msg)).await;
    
    // 监听弹幕消息
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Binary(data)) => {
                    // 解析弹幕数据
                    if let Some(danmaku) = parse_danmaku(&data) {
                        // 发送到前端（通过 Tauri 事件）
                        tracing::debug!("收到弹幕：{}", danmaku.content);
                    }
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("弹幕连接已关闭");
                    break;
                }
                Err(e) => {
                    tracing::error!("弹幕接收错误：{}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    Ok(())
}

/// 停止接收弹幕
pub async fn stop_danmaku(_room_id: &str) -> Result<()> {
    // TODO: 实现停止逻辑
    Ok(())
}

/// 获取弹幕服务器地址
async fn get_danmaku_server(room_id: &str) -> Result<(String, String)> {
    // 简化实现，实际需要从 API 获取
    Ok(("broadcastlv.chat.bilibili.com".to_string(), room_id.to_string()))
}

/// 构建加入房间消息
fn build_join_message(room_id: &str) -> Vec<u8> {
    // 构建 B 站弹幕协议
    let msg = format!("{{\"roomid\":{}}}", room_id);
    let mut buffer = Vec::new();
    
    // 协议头（16 字节）
    let packet_len = 16 + msg.len() as u32;
    buffer.extend_from_slice(&packet_len.to_be_bytes()); // 包长度
    buffer.extend_from_slice(&(16u16).to_be_bytes());   // 头长度
    buffer.extend_from_slice(&(1u16).to_be_bytes());    // 协议版本
    buffer.extend_from_slice(&(7u32).to_be_bytes());    // 操作类型（加入房间）
    buffer.extend_from_slice(&(1u32).to_be_bytes());    // 序列号
    
    // 消息体
    buffer.extend_from_slice(msg.as_bytes());
    
    buffer
}

/// 解析弹幕数据
fn parse_danmaku(data: &[u8]) -> Option<crate::commands::DanmakuMessage> {
    // 简化实现，实际需要解析 Protobuf
    None
}
