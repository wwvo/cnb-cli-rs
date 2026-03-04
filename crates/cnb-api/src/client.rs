//! CNB API HTTP 客户端

use crate::error::ApiError;
use crate::types::{Repo, User};
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};

/// CNB API 客户端
pub struct CnbClient {
    http: reqwest::Client,
    base_url: String,
    base_web_url: String,
    token: String,
    repo: String,
}

impl CnbClient {
    /// 创建新的 CNB API 客户端
    pub fn new(base_url: &str, base_web_url: &str, token: &str, repo: &str) -> Result<Self, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.cnb.api+json"));
        if !token.is_empty() {
            let auth_value = format!("Bearer {token}");
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value).map_err(|e| ApiError::Auth(e.to_string()))?,
            );
        }

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.to_string(),
            base_web_url: base_web_url.to_string(),
            token: token.to_string(),
            repo: repo.to_string(),
        })
    }

    /// 获取 API 基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 获取 Web 基础 URL
    pub fn base_web_url(&self) -> &str {
        &self.base_web_url
    }

    /// 获取当前仓库名
    pub fn repo(&self) -> &str {
        &self.repo
    }

    /// 获取 Token
    pub fn token(&self) -> &str {
        &self.token
    }

    /// 获取当前用户信息
    pub async fn me(&self) -> Result<User, ApiError> {
        let url = format!("{}user", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库信息
    pub async fn get_repo(&self) -> Result<Repo, ApiError> {
        let url = format!("{}{}", self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 处理 HTTP 响应，返回反序列化后的结果或错误
    async fn handle_response<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T, ApiError> {
        let status = resp.status().as_u16();
        if status < 200 || status >= 300 {
            let body = resp.text().await.unwrap_or_default();
            return Err(ApiError::HttpStatus { status, body });
        }
        let data = resp.json::<T>().await?;
        Ok(data)
    }
}
