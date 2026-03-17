//! version 子命令

/// 打印详细版本信息
pub fn run() {
    println!("{}", crate::build_info::COMMAND_LONG_VERSION);
}
