//! cnb commit asset-stats 子命令 - 统计 Commit 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;
use cnb_tui::fmt::format_bytes;
use futures::future::try_join_all;

/// 执行 commit asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let asset_lists = try_join_all(
        commits.iter().map(|c| client.get_commit_assets(&c.sha)),
    )
    .await?;

    let total_size: i64 = asset_lists.iter().flatten().map(|a| a.size_in_byte).sum();

    println!("Total Size: {total_size} ({})", format_bytes(total_size));

    Ok(())
}
