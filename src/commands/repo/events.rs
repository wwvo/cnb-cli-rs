//! cnb repo events 子命令 — 获取仓库动态

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;

/// 获取仓库动态内容
#[derive(Debug, Parser)]
pub struct EventsArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,

    /// 查询日期（格式：YYYY-MM-DD，默认今天）
    #[arg(short, long)]
    pub date: Option<String>,

    /// 指定小时
    #[arg(long)]
    pub hour: Option<u8>,
}

pub async fn run(ctx: &AppContext, args: &EventsArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    // 确定查询日期
    let date_str = match &args.date {
        Some(d) => d.clone(),
        None => {
            let now = chrono::Local::now();
            now.format("%Y-%m-%d").to_string()
        }
    };

    // 拼接日期参数（可附加小时）
    let date_param = match args.hour {
        Some(h) => format!("{date_str}-{h}"),
        None => date_str.clone(),
    };

    let client = ctx.api_client()?;
    let events = client.get_events(repo_path, &date_param).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&events)?);
        return Ok(());
    }

    // 动态内容直接输出
    if events.is_null() || (events.is_array() && events.as_array().is_some_and(|a| a.is_empty())) {
        info!("没有动态数据 ({date_str})");
        return Ok(());
    }

    println!("{}", serde_json::to_string_pretty(&events)?);

    Ok(())
}
