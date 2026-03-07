//! cnb build status 子命令 - 查询构建状态

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查询构建状态
#[derive(Debug, Parser)]
pub struct StatusArgs {
    /// 构建号
    pub sn: String,
}

/// 执行 build status 命令
pub async fn run(ctx: &AppContext, args: &StatusArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let result = client.get_build_status(&args.sn).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    println!("构建号:   {}", args.sn);
    println!("状态:     {}", result.status);

    if !result.pipelines_status.is_empty() {
        println!("流水线状态:");
        for (name, status) in &result.pipelines_status {
            println!("  {name}: {status}");
        }
    }

    Ok(())
}
