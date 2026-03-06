//! cnb repo top-contributors 子命令 — 查看仓库活跃用户排名

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看仓库活跃用户排名
#[derive(Debug, Parser)]
pub struct TopContributorsArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,

    /// 返回用户数量
    #[arg(short = 'n', long, default_value_t = 10)]
    pub top: u32,
}

pub async fn run(ctx: &AppContext, args: &TopContributorsArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let users = client.get_top_contributors(repo_path, args.top).await?;

    if users.is_empty() {
        info!("没有活跃用户数据");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&users)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("#", 4),
        Column::new("用户名", 18),
        Column::new("昵称", 15),
        Column::new("仓库数", 8),
        Column::new("⭐", 8),
    ]);

    for (i, user) in users.iter().enumerate() {
        table.add_row(vec![
            (i + 1).to_string(),
            user.username.clone(),
            user.nickname.clone(),
            user.repo_count.to_string(),
            user.stars_count.to_string(),
        ]);
    }

    table.print();

    Ok(())
}
