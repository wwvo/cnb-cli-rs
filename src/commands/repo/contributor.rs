//! cnb repo contributor 子命令 — 查看贡献者趋势数据

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看仓库贡献者排行及趋势数据
#[derive(Debug, Parser)]
pub struct ContributorArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,

    /// 贡献者数量上限
    #[arg(short = 'L', long, default_value_t = 30)]
    pub limit: u32,

    /// 排除外部用户
    #[arg(long, default_value_t = false)]
    pub exclude_external: bool,
}

pub async fn run(ctx: &AppContext, args: &ContributorArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let trend = client
        .get_contributor_trend(repo_path, args.limit.min(100), args.exclude_external)
        .await?;

    if trend.users_data.is_empty() {
        info!("没有贡献者数据");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&trend)?);
        return Ok(());
    }

    info!("共 {} 位贡献者", trend.user_total);

    let mut table = Table::new(vec![
        Column::new("用户", 20),
        Column::new("提交数", 8),
        Column::new("新增行", 10),
        Column::new("删除行", 10),
    ]);

    for user in &trend.users_data {
        let total_add: u64 = user.weeks.iter().map(|w| w.a).sum();
        let total_del: u64 = user.weeks.iter().map(|w| w.d).sum();

        table.add_row(vec![
            user.author.user_name.clone(),
            user.commit_count.to_string(),
            format!("+{total_add}"),
            format!("-{total_del}"),
        ]);
    }

    table.print();

    Ok(())
}
