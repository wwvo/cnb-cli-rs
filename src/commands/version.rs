//! version 子命令

/// 打印详细版本信息
pub fn run() {
    let version = env!("CARGO_PKG_VERSION");
    let target = option_env!("TARGET").unwrap_or("unknown");

    println!("cnb {version}");
    println!("target: {target}");
}
