//! cnb badge upload 子命令 - 上传自定义徽章

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UploadBadgeRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 上传自定义徽章
#[derive(Debug, Parser)]
pub struct UploadArgs {
    /// 徽章 key（如 security/tca）
    #[arg(short = 'k', long = "key")]
    pub key: String,

    /// Commit ID
    #[arg(long = "sha")]
    pub sha: String,

    /// 徽章右侧显示内容
    #[arg(short = 'm', long = "message")]
    pub message: String,

    /// 徽章数值
    #[arg(long = "value")]
    pub value: Option<i64>,

    /// 点击徽章右侧的跳转链接
    #[arg(short = 'l', long = "link")]
    pub link: Option<String>,

    /// 同时上传 latest 徽章
    #[arg(long = "latest", default_value_t = false)]
    pub latest: bool,
}

/// 执行 badge upload 命令
pub async fn run(ctx: &AppContext, args: &UploadArgs) -> Result<()> {
    let client = ctx.api_client()?;

    let req = UploadBadgeRequest {
        key: args.key.clone(),
        sha: args.sha.clone(),
        message: args.message.clone(),
        value: args.value,
        link: args.link.clone(),
        latest: if args.latest { Some(true) } else { None },
    };

    let result = client.upload_badge(&req).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    success!("徽章已上传");
    if !result.url.is_empty() {
        eprintln!("  Commit URL: {}", result.url);
    }
    if !result.latest_url.is_empty() {
        eprintln!("  Latest URL: {}", result.latest_url);
    }

    Ok(())
}
