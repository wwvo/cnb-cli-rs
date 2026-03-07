//! 构建相关类型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 触发构建请求
#[derive(Debug, Serialize)]
pub struct StartBuildRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<String>,
}

/// 构建触发/停止结果
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildResult {
    #[serde(default)]
    pub sn: String,
    #[serde(default, rename = "buildLogUrl")]
    pub build_log_url: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub success: bool,
}

/// 构建状态结果
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildStatusResult {
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "pipelinesStatus")]
    pub pipelines_status: HashMap<String, serde_json::Value>,
}

/// 构建列表结果
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildLogsResult {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub init: bool,
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub data: Vec<BuildLogEntry>,
}

/// 构建记录条目
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildLogEntry {
    #[serde(default)]
    pub sn: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub event: String,
    #[serde(default, rename = "eventUrl")]
    pub event_url: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default, rename = "sourceRef")]
    pub source_ref: String,
    #[serde(default, rename = "targetRef")]
    pub target_ref: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default, rename = "sourceSlug")]
    pub source_slug: String,
    #[serde(default, rename = "userName")]
    pub user_name: String,
    #[serde(default, rename = "nickName")]
    pub nick_name: String,
    #[serde(default, rename = "commitTitle")]
    pub commit_title: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub labels: String,
    #[serde(default, rename = "createTime")]
    pub create_time: String,
    #[serde(default)]
    pub duration: i64,
    #[serde(default, rename = "buildLogUrl")]
    pub build_log_url: String,
    #[serde(default, rename = "pipelineTotalCount")]
    pub pipeline_total_count: i32,
    #[serde(default, rename = "pipelineSuccessCount")]
    pub pipeline_success_count: i32,
    #[serde(default, rename = "pipelineFailCount")]
    pub pipeline_fail_count: i32,
}

/// Stage 详情结果
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildStageResult {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, rename = "startTime")]
    pub start_time: i64,
    #[serde(default, rename = "endTime")]
    pub end_time: i64,
    #[serde(default)]
    pub duration: i64,
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub content: Vec<String>,
}

/// 通用构建操作结果
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildCommonResult {
    #[serde(default)]
    pub code: i32,
    #[serde(default)]
    pub message: String,
}

/// 构建列表查询参数
pub struct BuildListOptions {
    pub page: u32,
    pub page_size: u32,
    pub status: Option<String>,
    pub event: Option<String>,
    pub source_ref: Option<String>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub end_time: Option<String>,
}
