//! cnb group create 子命令 - 创建组织

use anyhow::Result;
use clap::Parser;
use cnb_api::types::CreateGroupRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 创建组织
#[derive(Debug, Parser)]
pub struct CreateArgs {
    /// 组织路径（唯一标识）
    #[arg(value_name = "PATH")]
    pub path: String,

    /// 组织描述
    #[arg(short = 'd', long)]
    pub description: Option<String>,

    /// 备注
    #[arg(short = 'r', long)]
    pub remark: Option<String>,

    /// 根组织绑定的域名
    #[arg(long = "bind-domain")]
    pub bind_domain: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &CreateArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = CreateGroupRequest {
        path: args.path.clone(),
        description: args.description.clone(),
        remark: args.remark.clone(),
        bind_domain: args.bind_domain.clone(),
    };

    client.create_group(&req).await?;
    success!("组织 {} 创建成功", args.path);

    Ok(())
}
