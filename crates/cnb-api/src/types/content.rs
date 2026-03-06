//! 仓库文件内容相关类型定义

use serde::Deserialize;
use std::fmt;

/// 内容条目类型
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Blob,
    Lfs,
    Tree,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Blob => write!(f, "blob"),
            ContentType::Lfs => write!(f, "lfs"),
            ContentType::Tree => write!(f, "tree"),
        }
    }
}

/// 文件内容响应
#[derive(Debug, Deserialize)]
pub struct Content {
    /// 路径
    pub path: String,
    /// 类型
    #[serde(rename = "type")]
    pub content_type: ContentType,
    /// 文件内容（base64 编码，仅 blob 类型）
    #[serde(default)]
    pub content: String,
    /// LFS 下载 URL（仅 lfs 类型）
    #[serde(default)]
    pub lfs_download_url: String,
    /// 文件大小
    #[serde(default)]
    pub size: i64,
    /// 子条目（仅 tree 类型）
    #[serde(default)]
    pub entries: Vec<ContentEntry>,
}

/// 目录条目
#[derive(Debug, Deserialize)]
pub struct ContentEntry {
    /// 路径
    pub path: String,
    /// 类型
    #[serde(rename = "type")]
    pub entry_type: ContentType,
}
