//! cnb mission create 子命令 - 创建任务集

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateMissionRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 创建任务集
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// 组织路径
    #[arg(short = 'g', long = "group")]
    pub group: String,

    /// 任务集名称
    #[arg(short = 'n', long = "name")]
    pub name: String,

    /// 描述
    #[arg(short = 'd', long = "description")]
    pub description: Option<String>,

    /// 关联仓库列表（逗号分隔）
    #[arg(long = "repos", value_delimiter = ',')]
    pub repos: Option<Vec<String>>,

    /// 可见性（public/private/secret）
    #[arg(long = "visibility")]
    pub visibility: Option<String>,
}

/// 执行 mission create 命令
pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateMissionRequest {
        name: args.name.clone(),
        description: args.description.clone(),
        repos: args.repos.clone(),
        visibility: args.visibility.clone(),
    };

    client.create_mission(&args.group, &req).await?;
    success!("任务集 {} 已创建", args.name);

    Ok(())
}
