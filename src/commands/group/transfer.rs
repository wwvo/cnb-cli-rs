//! cnb group transfer 子命令 - 转移组织

use anyhow::Result;
use clap::Parser;
use cnb_api::types::TransferGroupRequest;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 转移组织到另一个父组织下
#[derive(Debug, Parser)]
pub struct TransferArgs {
    /// 要转移的组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 目标父组织路径
    #[arg(short = 't', long = "target")]
    pub target: String,

    /// 确认转移（跳过交互确认）
    #[arg(long)]
    pub confirm: bool,
}

pub async fn run(ctx: &AppContext, args: &TransferArgs) -> Result<()> {
    if !confirm_action(
        &format!(
            "确认将组织 \"{}\" 转移到 \"{}\" 下？",
            args.group, args.target
        ),
        args.confirm,
    )? {
        return Ok(());
    }

    let client = ctx.api_client()?;

    let req = TransferGroupRequest {
        source: args.group.clone(),
        target: args.target.clone(),
    };

    client.transfer_group(&args.group, &req).await?;
    success!("组织 {} 已转移到 {} 下", args.group, args.target);

    Ok(())
}
