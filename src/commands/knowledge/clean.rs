//! cnb knowledge clean 子命令

use anyhow::Result;
use cnb_core::context::AppContext;

/// 清除知识库
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;

    match client.delete_knowledge_base().await {
        Ok(()) => println!("知识库已清除"),
        Err(cnb_api::error::ApiError::NotFound(msg)) => println!("{msg}"),
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
