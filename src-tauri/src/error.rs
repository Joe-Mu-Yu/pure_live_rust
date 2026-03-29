//! 错误处理模块

use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error("网络请求失败：{0}")]
    NetworkError(String),

    #[error("解析失败：{0}")]
    ParseError(String),

    #[error("播放器错误：{0}")]
    PlayerError(String),

    #[error("数据库错误：{0}")]
    DatabaseError(String),

    #[error("未找到：{0}")]
    NotFound(String),

    #[error("认证失败：{0}")]
    AuthError(String),

    #[error("其他错误：{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
