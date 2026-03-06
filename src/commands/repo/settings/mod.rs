//! cnb repo settings 子命令组 — 仓库设置管理

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod branch_protection;
pub mod pipeline;
pub mod pull_request;
pub mod push_limit;

/// 仓库设置管理
#[derive(Debug, Parser)]
pub struct SettingsCommand {
    #[command(subcommand)]
    pub subcommand: SettingsSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum SettingsSubcommand {
    /// 分支保护规则管理
    #[command(name = "branch-protection")]
    BranchProtection(branch_protection::BranchProtectionCommand),

    /// 流水线构建设置
    Pipeline(pipeline::PipelineCommand),

    /// 合并请求设置
    #[command(name = "pull-request")]
    PullRequest(pull_request::PullRequestCommand),

    /// 推送限制设置
    #[command(name = "push-limit")]
    PushLimit(push_limit::PushLimitCommand),
}

impl SettingsCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            SettingsSubcommand::BranchProtection(cmd) => cmd.execute(ctx).await,
            SettingsSubcommand::Pipeline(cmd) => cmd.execute(ctx).await,
            SettingsSubcommand::PullRequest(cmd) => cmd.execute(ctx).await,
            SettingsSubcommand::PushLimit(cmd) => cmd.execute(ctx).await,
        }
    }
}
