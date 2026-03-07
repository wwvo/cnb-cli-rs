//! cnb registry set-visibility 子命令 - 设置制品库可见性

use anyhow::Result;
use clap::Parser;
use cnb_api::types::SetRegistryVisibilityRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 设置制品库可见性
#[derive(Debug, Parser)]
pub struct SetVisibilityArgs {
    /// 制品库路径
    pub registry: String,

    /// 可见性（public/private/secret）
    pub visibility: String,
}

/// 执行 registry set-visibility 命令
pub async fn run(ctx: &AppContext, args: &SetVisibilityArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = SetRegistryVisibilityRequest {
        visibility: args.visibility.clone(),
    };

    client.set_registry_visibility(&args.registry, &req).await?;
    success!("制品库可见性已设置为 {}", args.visibility);

    Ok(())
}
