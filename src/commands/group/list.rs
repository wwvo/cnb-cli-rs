//! cnb group list 子命令 - 列出我的组织

use anyhow::Result;
use clap::Parser;
use cnb_api::types::ListGroupsOptions;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 列出我的组织
#[derive(Debug, Parser)]
pub struct ListArgs {
    /// 指定父组织路径（查看子组织）
    #[arg(short = 'g', long = "group")]
    pub group: Option<String>,

    /// 关键字过滤
    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    /// 按角色过滤（Guest/Reporter/Developer/Master/Owner）
    #[arg(short = 'r', long = "role")]
    pub role: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &ListArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let opts = ListGroupsOptions {
        page: 1,
        page_size: 100,
        search: args.search.clone(),
        role: args.role.clone(),
    };

    let groups = if let Some(ref parent) = args.group {
        client.list_groups_under(parent, &opts).await?
    } else {
        client.list_top_groups(&opts).await?
    };

    if groups.is_empty() {
        info!("没有找到组织");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&groups)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("路径", 30),
        Column::new("描述", 30),
        Column::new("仓库", 6),
        Column::new("成员", 6),
        Column::new("子组织", 6),
    ]);

    for g in &groups {
        table.add_row(vec![
            g.path.clone(),
            g.description.clone(),
            g.sub_repo_count.to_string(),
            g.member_count.to_string(),
            g.sub_group_count.to_string(),
        ]);
    }

    table.print();

    Ok(())
}
