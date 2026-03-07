//! cnb workspace detail 子命令 - 查看工作区详情

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

/// 查看工作区详情
#[derive(Debug, Parser)]
pub struct DetailArgs {
    /// 流水线构建号
    #[arg(long = "sn")]
    pub sn: String,
}

/// 执行 workspace detail 命令
pub async fn run(ctx: &AppContext, args: &DetailArgs) -> Result<()> {
    let client = ctx.api_client()?;
    let repo = ctx.repo()?;

    let detail = client.get_workspace_detail(repo, &args.sn).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&detail)?);
        return Ok(());
    }

    if !detail.webide.is_empty() {
        println!("WebIDE:     {}", detail.webide);
    }
    if !detail.vscode.is_empty() {
        println!("VSCode:     {}", detail.vscode);
    }
    if !detail.vscode_insiders.is_empty() {
        println!("VSCode-Ins: {}", detail.vscode_insiders);
    }
    if !detail.cursor.is_empty() {
        println!("Cursor:     {}", detail.cursor);
    }
    if !detail.codebuddy.is_empty() {
        println!("CodeBuddy:  {}", detail.codebuddy);
    }
    if !detail.codebuddycn.is_empty() {
        println!("CodeBuddyCN:{}", detail.codebuddycn);
    }
    if !detail.ssh.is_empty() {
        println!("SSH:        {}", detail.ssh);
    }
    if !detail.remote_ssh.is_empty() {
        println!("RemoteSSH:  {}", detail.remote_ssh);
    }
    if !detail.jump_url.is_empty() {
        println!("JumpURL:    {}", detail.jump_url);
    }

    Ok(())
}
