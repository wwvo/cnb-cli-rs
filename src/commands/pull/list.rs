//! cnb pull list 子命令 - 列出与我相关的 Pull Request

use anyhow::Result;
use cnb_api::types::ListPullsOptions;
use cnb_core::context::AppContext;
use cnb_tui::{Column, Table};

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

    let mut table = Table::new(vec![
        Column::new("NUMBER", 15),
        Column::new("TITLE", 55),
        Column::new("BLOCKEDON", 15),
        Column::new("TYPE", 10),
    ]);
    for (number, title, blocked_on, pr_type) in &results {
        table.add_row(vec![
            format!("#{number}"),
            title.to_string(),
            blocked_on.to_string(),
            pr_type.to_string(),
        ]);
    }
    table.print();

    Ok(())
}
