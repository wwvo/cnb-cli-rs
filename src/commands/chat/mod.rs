//! Chat AI 子命令
//!
//! 使用自然语言与 CNB OpenAPI 交互。

use clap::Args;

pub mod agent;
pub mod stream;

/// Chat 命令参数
#[derive(Debug, Args)]
pub struct ChatArgs {
    /// 一次性模式：执行单个请求后退出（如 --do "查看 issue 列表"）
    #[arg(long, value_name = "问题")]
    pub do_: Option<String>,

    /// 禁用流式输出（适合 CI 环境）
    #[arg(long)]
    pub no_stream: bool,
}
