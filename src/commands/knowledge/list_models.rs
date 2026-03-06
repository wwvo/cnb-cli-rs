//! cnb knowledge list-models 子命令

use anyhow::Result;
use cnb_core::context::AppContext;

/// 列出支持的 Embedding 模型
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let models = client.list_knowledge_models().await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&models)?);
        return Ok(());
    }

    for model in &models {
        println!("{}", model.name);
    }

    Ok(())
}
