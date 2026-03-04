//! Knowledge 知识库子命令组

use clap::Parser;

pub mod clean;
pub mod info;
pub mod list_models;
pub mod query;

/// 知识库管理
#[derive(Debug, Parser)]
pub struct KnowledgeCommand {
    #[command(subcommand)]
    pub subcommand: KnowledgeSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum KnowledgeSubcommand {
    /// 列出支持的 Embedding 模型
    #[command(name = "list-models")]
    ListModels,
    /// 获取知识库信息
    Info,
    /// 清除知识库
    Clean,
    /// 查询知识库
    Query(query::QueryArgs),
}
