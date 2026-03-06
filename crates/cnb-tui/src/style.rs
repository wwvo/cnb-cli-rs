//! 终端样式辅助函数
//!
//! 提供 ANSI 转义码封装，支持 NO_COLOR 环境变量。

use std::sync::OnceLock;

/// 检查是否禁用颜色输出（NO_COLOR 环境变量）
fn no_color() -> bool {
    static NO_COLOR: OnceLock<bool> = OnceLock::new();
    *NO_COLOR.get_or_init(|| std::env::var("NO_COLOR").is_ok())
}

/// 灰色文本（用于辅助信息、思考过程等）
pub fn dim(text: &str) -> String {
    if no_color() {
        text.to_string()
    } else {
        format!("\x1b[90m{text}\x1b[0m")
    }
}

/// 清除当前行
pub fn clear_line() -> &'static str {
    if no_color() { "\r" } else { "\r\x1b[K" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_with_color() {
        // 在测试环境中 NO_COLOR 通常未设置
        let result = dim("hello");
        assert!(result.contains("hello"));
    }

    #[test]
    fn clear_line_not_empty() {
        let result = clear_line();
        assert!(!result.is_empty());
    }
}
