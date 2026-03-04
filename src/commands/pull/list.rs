//! cnb pull list 子命令 - 列出与我相关的 Pull Request

use anyhow::Result;
use cnb_api::types::ListPullsOptions;
use cnb_core::context::AppContext;

/// 执行 pull list 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let me = client.me().await?;

    // 获取我发起的 PR
    let from_me_opts = ListPullsOptions {
        state: "open".to_string(),
        page: 1,
        page_size: 100,
        authors: Some(me.username.clone()),
        ..Default::default()
    };
    let from_me = client.list_pulls(&from_me_opts).await.unwrap_or_default();

    // 获取需要我审查的 PR
    let to_me_opts = ListPullsOptions {
        state: "open".to_string(),
        page: 1,
        page_size: 100,
        reviewers: Some(me.username.clone()),
        ..Default::default()
    };
    let to_me = client.list_pulls(&to_me_opts).await.unwrap_or_default();

    // 合并并标记类型
    let mut results: Vec<(&str, &str, &str, &str)> = Vec::new();
    for pr in &from_me {
        results.push((&pr.number, &pr.title, &pr.blocked_on, "ME->"));
    }
    for pr in &to_me {
        results.push((&pr.number, &pr.title, &pr.blocked_on, "->ME"));
    }

    if results.is_empty() {
        println!("没有找到与我相关的 Pull Request");
        return Ok(());
    }

    println!(
        "{:<15} {:<55} {:<15} {:<10}",
        "NUMBER", "TITLE", "BLOCKEDON", "TYPE"
    );
    for (number, title, blocked_on, pr_type) in &results {
        let title = truncate_str(title, 52);
        println!(
            "{:<15} {:<55} {:<15} {:<10}",
            format!("#{number}"),
            title,
            blocked_on,
            pr_type
        );
    }

    Ok(())
}

/// UTF-8 安全的字符串截断
fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max_chars {
        let truncated: String = chars[..max_chars].iter().collect();
        format!("{truncated}...")
    } else {
        s.to_string()
    }
}
