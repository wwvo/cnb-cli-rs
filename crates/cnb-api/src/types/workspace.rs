//! 云原生工作区相关类型定义

use serde::{Deserialize, Serialize};

/// 工作区列表响应
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceListResponse {
    /// 总数
    #[serde(default)]
    pub total: i64,
    /// 工作区列表
    #[serde(default)]
    pub list: Vec<WorkspaceInfo>,
}

/// 工作区信息
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceInfo {
    /// 仓库路径
    #[serde(default)]
    pub slug: String,
    /// Pipeline ID
    #[serde(default)]
    pub pipeline_id: String,
    /// 状态（running / closed）
    #[serde(default)]
    pub status: String,
    /// 分支
    #[serde(default)]
    pub branch: String,
    /// 流水线构建号
    #[serde(default)]
    pub sn: String,
    /// 创建时间
    #[serde(default)]
    pub create_time: String,
    /// 持续时间（毫秒）
    #[serde(default)]
    pub duration: i64,
    /// 备份提交数
    #[serde(default)]
    pub commit_count: i64,
    /// 备份文件数
    #[serde(default)]
    pub file_count: i64,
    /// 备份文件列表
    #[serde(default)]
    pub file_list: String,
    /// 最新 commit short hash
    #[serde(default)]
    pub latest_sha: String,
    /// 是否支持 SSH
    #[serde(default)]
    pub ssh: bool,
    /// 默认工作区路径
    #[serde(default)]
    pub workspace: String,
}

/// 工作区列表查询选项
pub struct ListWorkspacesOptions {
    pub status: Option<String>,
    pub slug: Option<String>,
    pub branch: Option<String>,
    pub page: i32,
    pub page_size: i32,
}

/// 启动工作区请求
#[derive(Debug, Serialize)]
pub struct StartWorkspaceRequest {
    pub branch: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
}

/// 启动工作区响应
#[derive(Debug, Deserialize, Serialize)]
pub struct StartWorkspaceResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub sn: String,
    #[serde(default)]
    pub message: String,
    #[serde(default, rename = "buildLogUrl")]
    pub build_log_url: String,
}

/// 停止工作区请求
#[derive(Debug, Serialize)]
pub struct StopWorkspaceRequest {
    #[serde(skip_serializing_if = "Option::is_none", rename = "pipelineId")]
    pub pipeline_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<String>,
}

/// 停止工作区响应
#[derive(Debug, Deserialize, Serialize)]
pub struct StopWorkspaceResponse {
    #[serde(default)]
    pub sn: String,
    #[serde(default)]
    pub message: String,
    #[serde(default, rename = "buildLogUrl")]
    pub build_log_url: String,
}

/// 删除工作区请求
#[derive(Debug, Serialize)]
pub struct DeleteWorkspaceRequest {
    #[serde(skip_serializing_if = "Option::is_none", rename = "pipelineId")]
    pub pipeline_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<String>,
}

/// 工作区详情响应
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceDetailResponse {
    #[serde(default)]
    pub webide: String,
    #[serde(default)]
    pub vscode: String,
    #[serde(default, rename = "vscode-insiders")]
    pub vscode_insiders: String,
    #[serde(default)]
    pub cursor: String,
    #[serde(default)]
    pub codebuddy: String,
    #[serde(default)]
    pub codebuddycn: String,
    #[serde(default)]
    pub ssh: String,
    #[serde(default, rename = "remoteSsh")]
    pub remote_ssh: String,
    #[serde(default, rename = "jumpUrl")]
    pub jump_url: String,
}
