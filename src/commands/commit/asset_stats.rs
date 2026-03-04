//! cnb commit asset-stats 子命令 - 统计 Commit 附件大小

use anyhow::Result;
use cnb_core::context::AppContext;

/// 执行 commit asset-stats 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let commits = client.list_all_commits().await?;

    let mut total_size: i64 = 0;

    for commit in &commits {
        let assets = client.get_commit_assets(&commit.sha).await?;
        if assets.is_empty() {
            continue;
        }
        for asset in &assets {
            total_size += asset.size_in_byte;
        }
    }

    println!("Total Size: {} ({})", total_size, format_bytes(total_size));

    Ok(())
}

/// 格式化字节数为人类可读格式
fn format_bytes(bytes: i64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{bytes} B")
    } else {
        format!("{size:.1} {}", units[unit_idx])
    }
}
