//! Commit 子命令组

use clap::Parser;

pub mod asset_stats;

/// Commit 管理
#[derive(Debug, Parser)]
pub struct CommitCommand {
    #[command(subcommand)]
    pub subcommand: CommitSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum CommitSubcommand {
    /// 统计 Commit 附件大小
    #[command(name = "asset-stats")]
    AssetStats,
}
