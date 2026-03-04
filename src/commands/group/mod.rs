//! Group 组织子命令组

use clap::Parser;

pub mod update_logo;

/// 组织管理
#[derive(Debug, Parser)]
pub struct GroupCommand {
    #[command(subcommand)]
    pub subcommand: GroupSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum GroupSubcommand {
    /// 更新组织 Logo
    #[command(name = "update-logo")]
    UpdateLogo(update_logo::UpdateLogoArgs),
}
