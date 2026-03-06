//! 统一输出辅助宏
//!
//! 所有辅助信息输出到 stderr，保持 stdout 干净以支持管道/脚本消费。
//! stdout 仅用于数据输出（表格、URL、查询值等）。

/// 成功信息（stderr，✓ 前缀）
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => { eprintln!("✓ {}", format_args!($($arg)*)) };
}

/// 失败信息（stderr，✗ 前缀）
#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => { eprintln!("✗ {}", format_args!($($arg)*)) };
}

/// 警告信息（stderr，⚠ 前缀）
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => { eprintln!("⚠ {}", format_args!($($arg)*)) };
}

/// 普通辅助信息（stderr，无前缀）
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => { eprintln!("{}", format_args!($($arg)*)) };
}
