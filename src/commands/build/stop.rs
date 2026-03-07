//! cnb build stop 子命令 - 停止构建

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 停止构建
#[derive(Debug, Parser)]
pub struct StopArgs {
    /// 构建号
    pub sn: String,
}

/// 执行 build stop 命令
pub async fn run(ctx: &AppContext, args: &StopArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let result = client.stop_build(&args.sn).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    success!("构建 {} 已停止", args.sn);
    if !result.message.is_empty() {
        eprintln!("  {}", result.message);
    }

    Ok(())
}
