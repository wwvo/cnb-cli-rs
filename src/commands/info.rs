//! info 子命令 - 显示仓库和用户信息

use anyhow::Result;
use cnb_core::context::AppContext;

/// 执行 info 命令
pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = ctx.api_client()?;

    let user = client.me().await?;
    let repo = client.get_repo().await?;

    println!("  {:<16} {}", "MyID", user.id);
    println!("  {:<16} {}", "MyUserName", user.username);
    println!("  {:<16} {}", "MyNickName", user.nickname);
    println!("  {:<16} {}", "MyEmail", user.email);
    println!("  {:<16} {}", "RepoName", repo.path);
    println!("  {:<16} {}", "RepoID", repo.id);
    println!("  {:<16} {}", "RepoLicense", repo.license);
    println!("  {:<16} {}", "RepoStars", repo.star_count);
    println!("  {:<16} {}", "RepoForks", repo.fork_count);
    println!("  {:<16} {}", "RepoDescription", repo.description);

    Ok(())
}
