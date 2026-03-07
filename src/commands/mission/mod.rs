//! 任务集管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod create;
pub mod delete;
pub mod list;
pub mod set_visibility;
pub mod view_add;
pub mod view_get;
pub mod view_list;
pub mod view_set;
pub mod view_sort;

/// 任务集管理
#[derive(Debug, Parser)]
pub struct MissionCommand {
    #[command(subcommand)]
    pub subcommand: MissionSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum MissionSubcommand {
    /// 列出组织下的任务集
    List(list::ListArgs),
    /// 创建任务集
    Create(create::CreateArgs),
    /// 删除任务集
    Delete(delete::DeleteArgs),
    /// 设置任务集可见性
    #[command(name = "set-visibility")]
    SetVisibility(set_visibility::SetVisibilityArgs),
    /// 视图管理
    View(ViewCommand),
}

/// 视图管理子命令
#[derive(Debug, Parser)]
pub struct ViewCommand {
    #[command(subcommand)]
    pub subcommand: ViewSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum ViewSubcommand {
    /// 列出视图
    List(view_list::ViewListArgs),
    /// 获取视图配置
    Get(view_get::ViewGetArgs),
    /// 设置视图配置
    Set(view_set::ViewSetArgs),
    /// 添加/修改视图
    Add(view_add::ViewAddArgs),
    /// 排序视图
    Sort(view_sort::ViewSortArgs),
}

impl MissionCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            MissionSubcommand::List(args) => list::run(ctx, args).await,
            MissionSubcommand::Create(args) => create::run(ctx, args).await,
            MissionSubcommand::Delete(args) => delete::run(ctx, args).await,
            MissionSubcommand::SetVisibility(args) => set_visibility::run(ctx, args).await,
            MissionSubcommand::View(cmd) => cmd.execute(ctx).await,
        }
    }
}

impl ViewCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            ViewSubcommand::List(args) => view_list::run(ctx, args).await,
            ViewSubcommand::Get(args) => view_get::run(ctx, args).await,
            ViewSubcommand::Set(args) => view_set::run(ctx, args).await,
            ViewSubcommand::Add(args) => view_add::run(ctx, args).await,
            ViewSubcommand::Sort(args) => view_sort::run(ctx, args).await,
        }
    }
}
