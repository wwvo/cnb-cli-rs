//! cnb knowledge info 子命令

use anyhow::Result;
use cnb_core::context::AppContext;

/// 获取知识库信息
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;
    let info = client.get_knowledge_base_info().await?;

    println!("{:<20} {}", "ID", info.id);
    println!("{:<20} {}", "LastCommitSha", info.last_commit_sha);
    println!("{:<20} {}", "Model", info.embedding_model.name);
    println!("{:<20} {}", "Include", info.include);
    println!("{:<20} {}", "Exclude", info.exclude);

    Ok(())
}
