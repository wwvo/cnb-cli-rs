//! cnb issue list 子命令

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 列出仓库的 Issue
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 过滤 N 天内没有活动的 Issue（0 表示不过滤）
    #[arg(short = 'd', long = "stale-days", default_value_t = 0)]
    pub stale_days: u32,
}

/// 执行 issue list 命令
pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let issues = client.list_all_issues("open").await?;

    // 过滤逻辑
    let filtered: Vec<_> = if args.stale_days > 0 {
        issues
            .into_iter()
            .filter(|issue| is_stale(&issue.last_acted_at, args.stale_days))
            .collect()
    } else {
        issues
    };

    if filtered.is_empty() {
        println!("没有找到符合条件的 Issue");
        return Ok(());
    }

    // 表格输出
    println!(
        "{:<10} {:<60} {:<25} {:<10}",
        "NUMBER", "TITLE", "LastActedAt", "StaleDays"
    );
    for issue in &filtered {
        let stale_days = calculate_stale_days(&issue.last_acted_at);
        let title = truncate_str(&issue.title, 57);
        println!(
            "{:<10} {:<60} {:<25} {:<10}",
            issue.number, title, issue.last_acted_at, stale_days
        );
    }

    Ok(())
}

/// 判断 Issue 是否超过指定天数未活动
fn is_stale(last_acted_at: &str, stale_days: u32) -> bool {
    if stale_days == 0 {
        return false;
    }
    let days = calculate_stale_days(last_acted_at);
    days >= stale_days
}

/// UTF-8 安全的字符串截断（按字符数而非字节数）
fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max_chars {
        let truncated: String = chars[..max_chars].iter().collect();
        format!("{truncated}...")
    } else {
        s.to_string()
    }
}

/// 计算不活跃天数
fn calculate_stale_days(last_acted_at: &str) -> u32 {
    // 解析 RFC3339 格式的时间字符串
    let Ok(last_time) = chrono::DateTime::parse_from_rfc3339(last_acted_at) else {
        return 0;
    };
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(last_time);
    duration.num_days().max(0) as u32
}
