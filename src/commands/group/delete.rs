//! cnb group delete 子命令 - 删除组织

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除组织
#[derive(Debug, Parser)]
pub struct DeleteArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 确认删除（跳过交互确认）
    #[arg(long)]
    pub confirm: bool,
}

pub async fn run(ctx: &AppContext, args: &DeleteArgs) -> Result<()> {
    if !confirm_action(
        &format!("确认删除组织 \"{}\"？此操作不可逆！", args.group),
        args.confirm,
    )? {
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.delete_group(&args.group).await?;
    success!("组织 {} 已删除", args.group);

    Ok(())
}
