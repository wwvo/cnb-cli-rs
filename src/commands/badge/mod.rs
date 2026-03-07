//! 徽章管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod get;
pub mod list;
pub mod upload;

/// 徽章管理
#[derive(Debug, Parser)]
pub struct BadgeCommand {
    #[command(subcommand)]
    pub subcommand: BadgeSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum BadgeSubcommand {
    /// 获取指定徽章
    Get(get::GetArgs),
    /// 列出仓库徽章
    List(list::ListArgs),
    /// 上传自定义徽章
    Upload(upload::UploadArgs),
}

impl BadgeCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            BadgeSubcommand::Get(args) => get::run(ctx, args).await,
            BadgeSubcommand::List(args) => list::run(ctx, args).await,
            BadgeSubcommand::Upload(args) => upload::run(ctx, args).await,
        }
    }
}
