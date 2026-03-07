//! GPG 密钥管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod list;

/// GPG 密钥管理
#[derive(Debug, Parser)]
pub struct GpgKeyCommand {
    #[command(subcommand)]
    pub subcommand: GpgKeySubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum GpgKeySubcommand {
    /// 列出 GPG 密钥
    List(list::ListArgs),
}

impl GpgKeyCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            GpgKeySubcommand::List(args) => list::run(ctx, args).await,
        }
    }
}
