//! 仓库相关类型

use serde::Deserialize;

/// 仓库信息
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub id: String,
    pub path: String,
    pub description: String,
    pub license: String,
    pub star_count: u64,
    pub fork_count: u64,
}
