//! 组织（Group）相关类型定义

use serde::{Deserialize, Serialize};

/// 上传组织 Logo 请求
#[derive(Debug, Serialize)]
pub struct UploadLogoRequest {
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: i64,
}

/// 上传组织 Logo 响应
#[derive(Debug, Deserialize)]
pub struct UploadLogoResponse {
    /// COS 上传 URL
    pub upload_url: String,
    /// 表单字段
    #[serde(default)]
    pub form: std::collections::HashMap<String, String>,
}
