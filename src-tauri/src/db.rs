//! 数据库模块（基于 SQLite）

use crate::error::Result;
use rusqlite::{Connection, params};
use tauri::AppHandle;
use std::path::PathBuf;

static DB: once_cell::sync::Lazy<once_cell::sync::OnceCell<DbConnection>> = 
    once_cell::sync::Lazy::new(|| once_cell::sync::OnceCell::new());

pub struct DbConnection {
    conn: Connection,
}

/// 初始化数据库
pub fn init_database(app_handle: &AppHandle) -> Result<()> {
    let db_path = get_db_path(app_handle)?;
    
    let conn = Connection::open(&db_path)?;
    
    // 创建表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS favorites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site TEXT NOT NULL,
            room_id TEXT NOT NULL,
            title TEXT NOT NULL,
            created_at INTEGER DEFAULT (strftime('%s', 'now')),
            UNIQUE(site, room_id)
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            site TEXT NOT NULL,
            room_id TEXT NOT NULL,
            title TEXT NOT NULL,
            watched_at INTEGER DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    DB.set(DbConnection { conn }).unwrap();
    
    tracing::info!("数据库初始化完成：{:?}", db_path);
    
    Ok(())
}

/// 获取数据库文件路径
fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let db_dir = app_data_dir.join("db");
    std::fs::create_dir_all(&db_dir)?;
    
    Ok(db_dir.join("pure_live.db"))
}

/// 保存收藏
pub async fn save_favorite(site: String, room_id: String, title: String) -> Result<()> {
    let db = DB.get().unwrap();
    
    db.conn.execute(
        "INSERT OR REPLACE INTO favorites (site, room_id, title) VALUES (?1, ?2, ?3)",
        params![site, room_id, title],
    )?;
    
    Ok(())
}

/// 取消收藏
pub async fn remove_favorite(site: String, room_id: String) -> Result<()> {
    let db = DB.get().unwrap();
    
    db.conn.execute(
        "DELETE FROM favorites WHERE site = ?1 AND room_id = ?2",
        params![site, room_id],
    )?;
    
    Ok(())
}

/// 获取收藏列表
pub async fn get_favorites() -> Result<Vec<crate::commands::RoomInfo>> {
    let db = DB.get().unwrap();
    
    let mut stmt = db.conn.prepare(
        "SELECT site, room_id, title FROM favorites ORDER BY created_at DESC"
    )?;
    
    let favorites = stmt.query_map([], |row| {
        Ok(crate::commands::RoomInfo {
            site: row.get(0)?,
            room_id: row.get(1)?,
            title: row.get(2)?,
            anchor_name: String::new(),
            avatar: String::new(),
            viewer_count: 0,
            is_living: true,
            area_name: String::new(),
        })
    })?;
    
    let mut result = Vec::new();
    for fav in favorites {
        result.push(fav?);
    }
    
    Ok(result)
}

/// 保存历史记录
pub async fn save_history(site: String, room_id: String, title: String) -> Result<()> {
    let db = DB.get().unwrap();
    
    db.conn.execute(
        "INSERT INTO history (site, room_id, title) VALUES (?1, ?2, ?3)",
        params![site, room_id, title],
    )?;
    
    // 清理旧记录（保留最近 100 条）
    db.conn.execute(
        "DELETE FROM history WHERE id NOT IN (SELECT id FROM history ORDER BY watched_at DESC LIMIT 100)",
        [],
    )?;
    
    Ok(())
}

/// 获取历史记录
pub async fn get_history() -> Result<Vec<crate::commands::RoomInfo>> {
    let db = DB.get().unwrap();
    
    let mut stmt = db.conn.prepare(
        "SELECT site, room_id, title FROM history ORDER BY watched_at DESC LIMIT 50"
    )?;
    
    let history = stmt.query_map([], |row| {
        Ok(crate::commands::RoomInfo {
            site: row.get(0)?,
            room_id: row.get(1)?,
            title: row.get(2)?,
            anchor_name: String::new(),
            avatar: String::new(),
            viewer_count: 0,
            is_living: true,
            area_name: String::new(),
        })
    })?;
    
    let mut result = Vec::new();
    for hist in history {
        result.push(hist?);
    }
    
    Ok(result)
}
