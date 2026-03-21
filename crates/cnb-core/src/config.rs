//! 配置系统
//!
//! 支持 TOML 配置文件 (`~/.cnb/config.toml`) 和环境变量。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::BTreeMap, path::Path};

/// 默认 CNB 域名
pub const DEFAULT_DOMAIN: &str = "cnb.cool";

/// 默认 HTTPS scheme
pub const DEFAULT_SCHEME: &str = "https";

/// 默认 Git 协议
pub const DEFAULT_GIT_PROTOCOL: &str = "https";

/// 配置文件名
pub const CONFIG_FILE: &str = "config.toml";

/// cnb-rs 配置
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// 默认域名
    pub domain: Option<String>,

    /// Git 协议偏好 (https/ssh)
    pub git_protocol: Option<String>,

    /// 认证配置
    pub auth: Option<AuthConfigToml>,
}

/// 认证配置（TOML 层）
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthConfigToml {
    /// 按主机名存储的 token
    #[serde(flatten)]
    pub hosts: BTreeMap<String, HostAuth>,
}

/// 单个主机的认证信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HostAuth {
    /// 当前激活的账号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// 按用户名存储的认证元数据。
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub users: BTreeMap<String, UserAuth>,
    /// 兼容旧版配置的明文 token。
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub legacy_token: Option<String>,
    /// 兼容旧版配置的用户名。
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub legacy_username: Option<String>,
}

/// 单个账号的认证元数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuth {
    /// Secret 实际存储位置。
    pub storage: AuthTokenStorage,
    /// 当 `storage = "config"` 时保留明文 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// 凭证存储类型。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthTokenStorage {
    /// 存在系统 keyring 中。
    Keyring,
    /// 明文存在配置文件中。
    #[default]
    Config,
}

impl UserAuth {
    /// 创建 keyring 模式账号配置。
    #[must_use]
    pub fn keyring() -> Self {
        Self {
            storage: AuthTokenStorage::Keyring,
            token: None,
        }
    }

    /// 创建配置文件模式账号配置。
    #[must_use]
    pub fn config(token: impl Into<String>) -> Self {
        Self {
            storage: AuthTokenStorage::Config,
            token: Some(token.into()),
        }
    }
}

impl HostAuth {
    /// 获取当前激活用户，兼容旧版字段。
    #[must_use]
    pub fn active_user(&self) -> Option<&str> {
        self.user
            .as_deref()
            .or_else(|| self.users.keys().next().map(String::as_str))
            .or(self.legacy_username.as_deref())
    }

    /// 返回该域名下保存的所有账号，按字母排序。
    #[must_use]
    pub fn usernames(&self) -> Vec<String> {
        if !self.users.is_empty() {
            return self.users.keys().cloned().collect();
        }

        self.legacy_username.clone().into_iter().collect()
    }

    /// 查找指定用户的认证信息，兼容旧版字段。
    #[must_use]
    pub fn user_auth(&self, username: &str) -> Option<UserAuth> {
        if let Some(user_auth) = self.users.get(username) {
            return Some(user_auth.clone());
        }

        if self.legacy_username.as_deref() == Some(username) {
            return self
                .legacy_token
                .clone()
                .filter(|token| !token.is_empty())
                .map(UserAuth::config);
        }

        None
    }

    /// 将旧版 `token` / `username` 迁移到新结构，便于后续写回。
    pub fn migrate_legacy(&mut self) {
        if let Some(username) = self.legacy_username.clone() {
            let entry = self.users.entry(username.clone()).or_insert_with(|| {
                match self.legacy_token.clone() {
                    Some(token) if !token.is_empty() => UserAuth::config(token),
                    _ => UserAuth::keyring(),
                }
            });

            if entry.storage == AuthTokenStorage::Config
                && entry.token.is_none()
                && let Some(token) = self.legacy_token.clone()
                && !token.is_empty()
            {
                entry.token = Some(token);
            }

            if self.user.is_none() {
                self.user = Some(username);
            }
        }

        self.legacy_token = None;
        self.legacy_username = None;
    }

    /// 设置当前激活账号并写入账号元数据。
    pub fn upsert_user(&mut self, username: &str, user_auth: UserAuth) {
        self.migrate_legacy();
        self.users.insert(username.to_string(), user_auth);
        self.user = Some(username.to_string());
    }

    /// 删除指定账号。
    pub fn remove_user(&mut self, username: &str) -> bool {
        self.migrate_legacy();
        let removed = self.users.remove(username).is_some();
        if removed && self.user.as_deref() == Some(username) {
            self.user = self.users.keys().next().cloned();
        }
        removed
    }

    /// 切换到指定账号。
    pub fn switch_user(&mut self, username: &str) -> bool {
        self.migrate_legacy();
        if self.users.contains_key(username) {
            self.user = Some(username.to_string());
            true
        } else {
            false
        }
    }

    /// 判断该域名是否仍保存账号。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.usernames().is_empty()
    }
}

impl Config {
    /// 加载配置文件，文件不存在时返回默认配置
    ///
    /// # Errors
    ///
    /// Returns an error if the config file cannot be read or parsed.
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 配置文件路径：~/.cnb/config.toml
    #[must_use]
    pub fn config_path() -> PathBuf {
        let home = cnb_home_dir();
        home.join(CONFIG_FILE)
    }

    /// 将当前配置写回默认路径。
    ///
    /// # Errors
    ///
    /// Returns an error if the config file cannot be written.
    pub fn save(&self) -> anyhow::Result<()> {
        Self::write_config(&Self::config_path(), self)
    }

    /// 支持的配置 key 列表
    pub const VALID_KEYS: &[&str] = &["domain", "git_protocol"];

    /// 获取配置项的值
    #[must_use]
    pub fn get_value(&self, key: &str) -> Option<&str> {
        match key {
            "domain" => self.domain.as_deref(),
            "git_protocol" => self.git_protocol.as_deref(),
            _ => None,
        }
    }

    /// 设置配置项的值并写入文件
    ///
    /// # Errors
    ///
    /// Returns an error if the config file cannot be read, validated, or written.
    pub fn set_value(key: &str, value: &str) -> anyhow::Result<()> {
        let path = Self::config_path();
        let mut config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str::<Config>(&content)?
        } else {
            Config::default()
        };

        match key {
            "domain" => config.domain = Some(value.to_string()),
            "git_protocol" => {
                if value != "https" {
                    anyhow::bail!("CNB 暂不支持 SSH 克隆，git_protocol 当前仅支持 https");
                }
                config.git_protocol = Some(value.to_string());
            }
            _ => anyhow::bail!(
                "未知配置项：{key}\n可用配置项：{}",
                Self::VALID_KEYS.join(", ")
            ),
        }

        Self::write_config(&path, &config)
    }

    /// 保存认证信息到配置文件
    ///
    /// 保留已有配置，仅更新指定域名的 auth 段。
    ///
    /// # Errors
    ///
    /// Returns an error if the config file cannot be read, parsed, or written.
    pub fn save_auth(domain: &str, token: &str, username: &str) -> anyhow::Result<()> {
        let mut config = Self::load()?;
        config.upsert_config_auth(domain, username, token);
        config.save()
    }

    /// 从配置文件中移除指定域名的认证信息
    ///
    /// # Errors
    ///
    /// Returns an error if the config file exists but cannot be read, parsed, or
    /// written back.
    pub fn remove_auth(domain: &str) -> anyhow::Result<bool> {
        let mut config = Self::load()?;
        let removed = config
            .auth
            .as_mut()
            .and_then(|auth| auth.hosts.remove(domain))
            .is_some();

        if removed {
            config.save()?;
        }

        Ok(removed)
    }

    /// 写入配置文件，自动创建父目录
    fn write_config(path: &Path, config: &Config) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// 获取指定域名的认证配置。
    #[must_use]
    pub fn host_auth(&self, domain: &str) -> Option<&HostAuth> {
        self.auth.as_ref().and_then(|auth| auth.hosts.get(domain))
    }

    /// 获取指定域名的认证配置并允许修改。
    pub fn host_auth_mut(&mut self, domain: &str) -> &mut HostAuth {
        self.auth
            .get_or_insert_with(AuthConfigToml::default)
            .hosts
            .entry(domain.to_string())
            .or_default()
    }

    /// 更新指定域名的明文账号信息。
    pub fn upsert_config_auth(&mut self, domain: &str, username: &str, token: &str) {
        self.host_auth_mut(domain)
            .upsert_user(username, UserAuth::config(token));
    }

    /// 更新指定域名的 keyring 账号信息。
    pub fn upsert_keyring_auth(&mut self, domain: &str, username: &str) {
        self.host_auth_mut(domain)
            .upsert_user(username, UserAuth::keyring());
    }

    /// 切换指定域名的激活账号。
    #[must_use]
    pub fn switch_auth_user(&mut self, domain: &str, username: &str) -> bool {
        self.auth
            .as_mut()
            .and_then(|auth| auth.hosts.get_mut(domain))
            .is_some_and(|host| host.switch_user(username))
    }

    /// 删除指定域名下的账号；域名下没有剩余账号时清理整段配置。
    #[must_use]
    pub fn remove_auth_user(&mut self, domain: &str, username: &str) -> bool {
        let Some(auth) = self.auth.as_mut() else {
            return false;
        };
        let Some(host) = auth.hosts.get_mut(domain) else {
            return false;
        };

        let removed = host.remove_user(username);
        if removed && host.is_empty() {
            let _ = auth.hosts.remove(domain);
        }

        removed
    }

    /// 列出指定域名下的所有账号。
    #[must_use]
    pub fn auth_usernames(&self, domain: &str) -> Vec<String> {
        self.host_auth(domain)
            .map_or_else(Vec::new, HostAuth::usernames)
    }
}

/// 获取 CNB 主目录：~/.cnb/
#[must_use]
pub fn cnb_home_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| {
            tracing::warn!("无法获取用户主目录，配置文件将使用当前目录下的 .cnb/");
            PathBuf::from(".")
        })
        .join(".cnb")
}

#[cfg(test)]
mod tests {
    use super::{AuthConfigToml, AuthTokenStorage, Config, HostAuth, UserAuth};
    use std::collections::BTreeMap;

    fn config_with_host(domain: &str, host: HostAuth) -> Config {
        Config {
            auth: Some(AuthConfigToml {
                hosts: BTreeMap::from([(domain.to_string(), host)]),
            }),
            ..Config::default()
        }
    }

    #[test]
    fn host_auth_migrates_legacy_single_user() {
        let mut host = HostAuth {
            legacy_token: Some("legacy-token".to_string()),
            legacy_username: Some("octocat".to_string()),
            ..HostAuth::default()
        };

        host.migrate_legacy();

        assert_eq!(host.user.as_deref(), Some("octocat"));
        assert_eq!(host.users.len(), 1);
        assert_eq!(
            host.users.get("octocat").map(|user| user.storage),
            Some(AuthTokenStorage::Config)
        );
        assert_eq!(
            host.users
                .get("octocat")
                .and_then(|user| user.token.as_deref()),
            Some("legacy-token")
        );
        assert!(host.legacy_token.is_none());
        assert!(host.legacy_username.is_none());
    }

    #[test]
    fn switch_auth_user_updates_active_user() {
        let mut config = config_with_host(
            "cnb.cool",
            HostAuth {
                user: Some("alice".to_string()),
                users: BTreeMap::from([
                    ("alice".to_string(), UserAuth::keyring()),
                    ("octocat".to_string(), UserAuth::config("token-1")),
                ]),
                ..HostAuth::default()
            },
        );

        assert!(config.switch_auth_user("cnb.cool", "octocat"));
        assert_eq!(
            config.host_auth("cnb.cool").and_then(HostAuth::active_user),
            Some("octocat")
        );
    }

    #[test]
    fn remove_auth_user_switches_to_remaining_user() {
        let mut config = config_with_host(
            "cnb.cool",
            HostAuth {
                user: Some("alice".to_string()),
                users: BTreeMap::from([
                    ("alice".to_string(), UserAuth::keyring()),
                    ("octocat".to_string(), UserAuth::config("token-1")),
                ]),
                ..HostAuth::default()
            },
        );

        assert!(config.remove_auth_user("cnb.cool", "alice"));
        assert_eq!(
            config.host_auth("cnb.cool").and_then(HostAuth::active_user),
            Some("octocat")
        );
    }
}
