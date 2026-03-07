//! GPG 密钥相关类型

use serde::{Deserialize, Serialize};

/// GPG 公钥
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GpgPublicKey {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub raw_key: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expired_at: String,
    #[serde(default)]
    pub emails: Vec<GpgKeyEmail>,
    #[serde(default)]
    pub subkeys: Vec<GpgSubKey>,
}

/// GPG 密钥关联邮箱
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GpgKeyEmail {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub verified: bool,
}

/// GPG 子密钥
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct GpgSubKey {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key_id: String,
    #[serde(default)]
    pub primary_key_id: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expired_at: String,
}
