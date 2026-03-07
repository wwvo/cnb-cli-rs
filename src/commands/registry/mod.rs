//! 制品库管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod delete;
pub mod list;
pub mod package_delete;
pub mod package_detail;
pub mod package_list;
pub mod set_visibility;
pub mod tag_delete;
pub mod tag_detail;
pub mod tag_list;

/// 制品库管理
#[derive(Debug, Parser)]
pub struct RegistryCommand {
    #[command(subcommand)]
    pub subcommand: RegistrySubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum RegistrySubcommand {
    /// 列出组织下的制品库
    List(list::ListArgs),
    /// 删除制品库
    Delete(delete::DeleteArgs),
    /// 设置制品库可见性
    #[command(name = "set-visibility")]
    SetVisibility(set_visibility::SetVisibilityArgs),
    /// 制品管理
    Package(PackageCommand),
    /// 标签管理
    Tag(TagCommand),
}

/// 制品管理子命令
#[derive(Debug, Parser)]
pub struct PackageCommand {
    #[command(subcommand)]
    pub subcommand: PackageSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum PackageSubcommand {
    /// 列出制品
    List(package_list::PackageListArgs),
    /// 查看制品详情
    Detail(package_detail::PackageDetailArgs),
    /// 删除制品
    Delete(package_delete::PackageDeleteArgs),
}

/// 标签管理子命令
#[derive(Debug, Parser)]
pub struct TagCommand {
    #[command(subcommand)]
    pub subcommand: TagSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum TagSubcommand {
    /// 列出制品标签
    List(tag_list::TagListArgs),
    /// 查看标签详情
    Detail(tag_detail::TagDetailArgs),
    /// 删除制品标签
    Delete(tag_delete::TagDeleteArgs),
}

impl RegistryCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            RegistrySubcommand::List(args) => list::run(ctx, args).await,
            RegistrySubcommand::Delete(args) => delete::run(ctx, args).await,
            RegistrySubcommand::SetVisibility(args) => set_visibility::run(ctx, args).await,
            RegistrySubcommand::Package(cmd) => cmd.execute(ctx).await,
            RegistrySubcommand::Tag(cmd) => cmd.execute(ctx).await,
        }
    }
}

impl PackageCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            PackageSubcommand::List(args) => package_list::run(ctx, args).await,
            PackageSubcommand::Detail(args) => package_detail::run(ctx, args).await,
            PackageSubcommand::Delete(args) => package_delete::run(ctx, args).await,
        }
    }
}

impl TagCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            TagSubcommand::List(args) => tag_list::run(ctx, args).await,
            TagSubcommand::Detail(args) => tag_detail::run(ctx, args).await,
            TagSubcommand::Delete(args) => tag_delete::run(ctx, args).await,
        }
    }
}
