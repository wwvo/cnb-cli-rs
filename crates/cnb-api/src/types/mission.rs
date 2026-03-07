//! 任务集管理相关类型

use serde::{Deserialize, Serialize};

/// 任务集信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Mission {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub visibility_level: serde_json::Value,
    #[serde(default)]
    pub access: serde_json::Value,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub pinned_time: String,
    #[serde(default)]
    pub stared: bool,
    #[serde(default)]
    pub star_time: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// 创建任务集请求
#[derive(Debug, Serialize)]
pub struct CreateMissionRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// 设置可见性请求
#[derive(Debug, Serialize)]
pub struct SetMissionVisibilityRequest {
    pub visibility: String,
}

/// 任务集视图
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MissionView {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "type")]
    pub view_type: serde_json::Value,
}

/// 任务集视图配置
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MissionViewConfig {
    #[serde(default)]
    pub id: String,
    #[serde(default, rename = "type")]
    pub view_type: serde_json::Value,
    #[serde(default)]
    pub fields: Vec<serde_json::Value>,
    #[serde(default)]
    pub selectors: Vec<serde_json::Value>,
    #[serde(default)]
    pub sorts: Vec<serde_json::Value>,
    #[serde(default)]
    pub group: serde_json::Value,
}
