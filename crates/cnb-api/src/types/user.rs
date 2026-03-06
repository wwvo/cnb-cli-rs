//! 用户相关类型

use serde::{Deserialize, Serialize};

/// 当前登录用户信息
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub email: String,
}
