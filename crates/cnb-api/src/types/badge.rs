//! 徽章相关类型

use serde::{Deserialize, Serialize};

/// 徽章数据
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BadgeResult {
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub links: Vec<String>,
}

/// 徽章列表项
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BadgeItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default, rename = "type")]
    pub badge_type: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub group: Option<BadgeGroup>,
}

/// 徽章分组
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BadgeGroup {
    #[serde(default, rename = "type")]
    pub group_type: String,
    #[serde(default, rename = "typeEn")]
    pub type_en: String,
    #[serde(default)]
    pub status: String,
}

/// 徽章列表结果
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BadgeListResult {
    #[serde(default)]
    pub badges: Vec<BadgeItem>,
}

/// 上传徽章请求
#[derive(Debug, Serialize)]
pub struct UploadBadgeRequest {
    pub key: String,
    pub sha: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
}

/// 上传徽章结果
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UploadBadgeResult {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub latest_url: String,
}
