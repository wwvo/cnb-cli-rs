//! cnb issue reopen 子命令 - 重新打开 Issue

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateIssueRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 重新打开已关闭的 Issue
#[derive(Debug, Parser)]
pub struct ReopenArgs {
    /// Issue 编号
    #[arg(value_name = "NUMBER")]
    pub number: String,
}

/// 执行 issue reopen 命令
pub async fn run(ctx: &AppContext, args: &ReopenArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = UpdateIssueRequest {
        state: Some("open".to_string()),
        state_reason: Some("reopened".to_string()),
        ..Default::default()
    };

    client.update_issue(&args.number, &req).await?;
    success!("Issue #{} 已重新打开", args.number);

    Ok(())
}
