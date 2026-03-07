//! cnb registry tag delete 子命令 - 删除制品标签

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::success;

/// 删除制品标签
#[derive(Debug, Parser)]
pub struct TagDeleteArgs {
    /// 制品类型（docker/npm/pypi/maven/helm 等）
    pub pkg_type: String,

    /// 制品名称
    pub name: String,

    /// 标签名
    pub tag: String,

    /// 制品库路径（组织/制品库）
    #[arg(short = 'r', long = "registry")]
    pub registry: String,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

/// 执行 registry tag delete 命令
pub async fn run(ctx: &AppContext, args: &TagDeleteArgs) -> Result<()> {
    let client = ctx.api_client()?;

    confirm_action(
        &format!("确认删除标签 {}/{}:{}？", args.pkg_type, args.name, args.tag),
        args.yes,
    )?;

    client.delete_package_tag(&args.registry, &args.pkg_type, &args.name, &args.tag).await?;
    success!("标签 {}/{}:{} 已删除", args.pkg_type, args.name, args.tag);

    Ok(())
}
