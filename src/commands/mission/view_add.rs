//! cnb mission view add 子命令 - 添加/修改视图

use anyhow::Result;
use clap::Parser;
use cnb_api::types::MissionView;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 添加/修改任务集视图
#[derive(Debug, Parser)]
pub struct ViewAddArgs {
    /// 任务集路径
    pub mission: String,

    /// 视图名称
    #[arg(long = "name")]
    pub name: String,

    /// 视图类型
    #[arg(long = "type")]
    pub view_type: String,

    /// 视图 ID（修改已有视图时指定）
    #[arg(long = "id")]
    pub view_id: Option<String>,
}

/// 执行 mission view add 命令
pub async fn run(ctx: &AppContext, args: &ViewAddArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let view = MissionView {
        id: args.view_id.clone().unwrap_or_default(),
        name: args.name.clone(),
        view_type: serde_json::Value::String(args.view_type.clone()),
    };

    client.put_mission_view(&args.mission, &[view]).await?;

    if args.view_id.is_some() {
        success!("视图已更新");
    } else {
        success!("视图已添加");
    }

    Ok(())
}
