//! 用户相关类型

use serde::{Deserialize, Serialize};

/// 当前登录用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub email: String,
}

/// 粉丝/关注项
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UserFollowResult {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub is_following: bool,
    #[serde(default)]
    pub freeze: bool,
    #[serde(default)]
    pub locked: bool,
}

/// 活动汇总
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ActivityDate {
    #[serde(default)]
    pub commit_count: i64,
    #[serde(default)]
    pub pull_request_count: i64,
    #[serde(default)]
    pub issues_count: i64,
    #[serde(default)]
    pub code_review_count: i64,
    #[serde(default)]
    pub repo_count: i64,
    #[serde(default)]
    pub group_count: i64,
    #[serde(default)]
    pub private_score: i64,
}
