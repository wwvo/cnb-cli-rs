//! 配置系统
//!
//! 支持 TOML 配置文件 (`~/.cnb/config.toml`) 和环境变量。

use serde::Deserialize;
use std::path::PathBuf;

/// 默认 CNB 域名
pub const DEFAULT_DOMAIN: &str = "cnb.cool";

/// 默认 HTTPS scheme
pub const DEFAULT_SCHEME: &str = "https";

/// 配置文件名
pub const CONFIG_FILE: &str = "config.toml";

/// CNB CLI 配置
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// 默认域名
    pub domain: Option<String>,

    /// Git 协议偏好 (https/ssh)
    pub git_protocol: Option<String>,

    /// 认证配置
    pub auth: Option<AuthConfigToml>,
}

/// 认证配置（TOML 层）
#[derive(Debug, Default, Deserialize)]
pub struct AuthConfigToml {
    /// 按主机名存储的 token
    #[serde(flatten)]
    pub hosts: std::collections::HashMap<String, HostAuth>,
}

/// 单个主机的认证信息
#[derive(Debug, Default, Deserialize)]
pub struct HostAuth {
    pub token: Option<String>,
    pub username: Option<String>,
}

impl Config {
    /// 加载配置文件，文件不存在时返回默认配置
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 配置文件路径: ~/.cnb/config.toml
    pub fn config_path() -> PathBuf {
        let home = cnb_home_dir();
        home.join(CONFIG_FILE)
    }
}

/// 获取 CNB 主目录: ~/.cnb/
pub fn cnb_home_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cnb")
}
