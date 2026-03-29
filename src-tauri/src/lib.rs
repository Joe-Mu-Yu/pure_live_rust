mod commands;
mod error;
// mod sites;       // TODO: 实现平台适配
// mod danmaku;     // TODO: 实现弹幕系统
// mod player;      // TODO: 实现播放器
// mod db;          // TODO: 实现数据库
// mod services;    // TODO: 实现外部服务

use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn run_app() {
    // 初始化日志系统
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pure_live_rust=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("启动纯粹直播 Rust 版...");

    // 初始化应用
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // TODO: 初始化数据库
            // db::init_database(app.handle())?;
            
            // TODO: 初始化播放器
            // player::init_player(app.handle())?;
            
            tracing::info!("应用初始化完成");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_room_info,
            commands::get_live_streams,
            commands::start_danmaku,
            commands::stop_danmaku,
            commands::play_stream,
            commands::pause_stream,
            commands::stop_player,
            commands::get_orientation,
            commands::save_favorite,
            commands::remove_favorite,
            commands::get_favorites,
            commands::get_history,
            commands::save_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
