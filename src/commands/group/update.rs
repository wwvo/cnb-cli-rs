//! cnb group update 子命令 - 更新组织信息

use anyhow::Result;
use clap::Parser;
use cnb_api::types::UpdateGroupRequest;
use cnb_core::context::AppContext;
use cnb_tui::success;

/// 更新组织信息
#[derive(Debug, Parser)]
pub struct UpdateArgs {
    /// 组织路径
    #[arg(value_name = "GROUP")]
    pub group: String,

    /// 更新描述
    #[arg(short = 'd', long)]
    pub description: Option<String>,

    /// 更新备注
    #[arg(short = 'r', long)]
    pub remark: Option<String>,

    /// 更新联系邮箱
    #[arg(short = 'e', long)]
    pub email: Option<String>,

    /// 更新网站
    #[arg(short = 's', long)]
    pub site: Option<String>,

    /// 更新域名
    #[arg(long)]
    pub domain: Option<String>,

    /// 更新微信公众号
    #[arg(long = "wechat-mp")]
    pub wechat_mp: Option<String>,

    /// 设置 README 仓库路径
    #[arg(long = "readme-repo")]
    pub readme_repo: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    if args.description.is_none()
        && args.remark.is_none()
        && args.email.is_none()
        && args.site.is_none()
        && args.domain.is_none()
        && args.wechat_mp.is_none()
        && args.readme_repo.is_none()
    {
        anyhow::bail!(
            "请至少指定一个修改项（--description、--remark、--email、--site、--domain、--wechat-mp、--readme-repo）"
        );
    }

    let client = ctx.api_client()?;

    let req = UpdateGroupRequest {
        description: args.description.clone(),
        remark: args.remark.clone(),
        email: args.email.clone(),
        site: args.site.clone(),
        domain: args.domain.clone(),
        wechat_mp: args.wechat_mp.clone(),
        readme_repo_path: args.readme_repo.clone(),
    };

    client.update_group(&args.group, &req).await?;
    success!("组织 {} 信息已更新", args.group);

    Ok(())
}
