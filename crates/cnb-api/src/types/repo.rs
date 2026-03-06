//! 仓库相关类型

use serde::{Deserialize, Serialize};

/// 仓库信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Repo {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub star_count: u64,
    #[serde(default)]
    pub fork_count: u64,
    #[serde(default)]
    pub open_issue_count: u64,
    #[serde(default)]
    pub open_pull_request_count: u64,
    #[serde(default)]
    pub visibility_level: Option<serde_json::Value>,
    #[serde(default)]
    pub languages: Option<RepoLanguage>,
    #[serde(default)]
    pub last_updated_at: Option<RepoTime>,
    #[serde(default)]
    pub web_url: String,
    #[serde(default)]
    pub stared: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub site: String,
    #[serde(default)]
    pub topics: String,
}

/// 仓库语言信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RepoLanguage {
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub language: String,
}

/// 仓库时间信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RepoTime {
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub valid: bool,
}

/// Fork 列表响应
#[derive(Debug, Deserialize, Serialize)]
pub struct ForkList {
    #[serde(default)]
    pub fork_tree_count: u64,
    #[serde(default)]
    pub forks: Vec<ForkInfo>,
}

/// Fork 信息
#[derive(Debug, Deserialize, Serialize)]
pub struct ForkInfo {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub fork_count: u64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub freeze: bool,
}

/// 创建仓库请求体
#[derive(Debug, Serialize)]
pub struct CreateRepoRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// 更新仓库请求体
#[derive(Debug, Serialize)]
pub struct UpdateRepoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

/// 贡献者趋势响应
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorTrend {
    #[serde(default)]
    pub user_total: u64,
    #[serde(default)]
    pub week_total: u64,
    #[serde(default)]
    pub users_data: Vec<ContributorData>,
    #[serde(default)]
    pub with_line_counts: bool,
}

/// 贡献者数据
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorData {
    #[serde(default)]
    pub author: ContributorAuthor,
    #[serde(default)]
    pub commit_count: u64,
    #[serde(default)]
    pub weeks: Vec<ContributorWeek>,
}

/// 贡献者作者信息
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContributorAuthor {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub user_name: String,
}

/// 贡献者周数据
#[derive(Debug, Deserialize, Serialize)]
pub struct ContributorWeek {
    #[serde(default)]
    pub a: u64,
    #[serde(default)]
    pub c: u64,
    #[serde(default)]
    pub d: u64,
    #[serde(default)]
    pub w: u64,
}

/// 安全概览响应
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityOverview {
    #[serde(default)]
    pub risk_cnt: Option<SecurityRiskCount>,
    #[serde(default)]
    pub code_sensitive: Option<SecurityModule>,
    #[serde(default)]
    pub code_vulnerability: Option<SecurityVulnerability>,
    #[serde(default)]
    pub code_issue: Option<SecurityModule>,
}

/// 安全风险计数
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityRiskCount {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub code_sensitive_enable: bool,
    #[serde(default)]
    pub code_sensitive_risk_cnt: u64,
    #[serde(default)]
    pub code_vulnerability_enable: bool,
    #[serde(default)]
    pub code_vulnerability_risk_cnt: u64,
    #[serde(default)]
    pub code_issue_enable: bool,
    #[serde(default)]
    pub code_issue_risk_cnt: u64,
}

/// 安全模块（敏感信息 / 代码问题）
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityModule {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub open: u64,
    #[serde(default)]
    pub ignored: u64,
    #[serde(default)]
    pub high_count: u64,
    #[serde(default)]
    pub medium_count: u64,
    #[serde(default)]
    pub low_count: u64,
    #[serde(default)]
    pub critical_count: u64,
}

/// 安全漏洞模块
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityVulnerability {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub open: u64,
    #[serde(default)]
    pub ignored: u64,
    #[serde(default)]
    pub critical_vul_open_cnt: u64,
    #[serde(default)]
    pub high_vul_open_cnt: u64,
    #[serde(default)]
    pub medium_vul_open_cnt: u64,
    #[serde(default)]
    pub low_vul_open_cnt: u64,
}

/// 活跃用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct TopContributor {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub bio: String,
    #[serde(default)]
    pub repo_count: u64,
    #[serde(default)]
    pub stars_count: u64,
}

/// 仓库资产记录
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetRecord {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub origin_path: String,
    #[serde(default)]
    pub record_type: Option<serde_json::Value>,
    #[serde(default)]
    pub referer: String,
    #[serde(default)]
    pub size_in_byte: u64,
}

/// 仓库列表查询选项
#[derive(Debug)]
pub struct ListReposOptions {
    pub page: u32,
    pub page_size: u32,
    pub search: Option<String>,
    pub filter_type: Option<String>,
    pub order_by: Option<String>,
    pub desc: bool,
}

impl Default for ListReposOptions {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 30,
            search: None,
            filter_type: None,
            order_by: None,
            desc: false,
        }
    }
}

impl ListReposOptions {
    /// 拼接通用查询参数
    pub fn query_string(&self) -> String {
        let mut params = vec![
            format!("page={}", self.page),
            format!("page_size={}", self.page_size),
        ];
        if let Some(s) = &self.search {
            params.push(format!("search={s}"));
        }
        if let Some(f) = &self.filter_type {
            params.push(format!("filter_type={f}"));
        }
        if let Some(o) = &self.order_by {
            params.push(format!("order_by={o}"));
        }
        if self.desc {
            params.push("desc=true".to_string());
        }
        params.join("&")
    }
}
