//! CNB API HTTP 客户端

mod ai;
mod commit;
mod group;
mod issue;
mod knowledge;
mod pull;
mod release;
mod star;
mod workspace;

use crate::error::ApiError;
use crate::types::*;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};

/// CNB API 客户端
pub struct CnbClient {
    pub(crate) http: reqwest::Client,
    pub(crate) http_plain: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) base_web_url: String,
    pub(crate) token: String,
    pub(crate) repo: String,
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

        let http_plain = reqwest::Client::new();

        Ok(Self {
            http,
            http_plain,
            base_url: base_url.to_string(),
            base_web_url: base_web_url.to_string(),
            token: token.to_string(),
            repo: repo.to_string(),
        })
    }

    pub fn base_url(&self) -> &str { &self.base_url }
    pub fn base_web_url(&self) -> &str { &self.base_web_url }
    pub fn repo(&self) -> &str { &self.repo }
    pub fn token(&self) -> &str { &self.token }

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

    /// 获取仓库文件/目录内容
    pub async fn get_content(&self, path: &str, git_ref: &str) -> Result<Content, ApiError> {
        let url = format!("{}{}/-/git/contents/{}?ref={git_ref}", self.base_url, self.repo, path);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    // ==================== Internal ====================

    pub(crate) async fn paginate<T, F, Fut>(&self, fetch: F) -> Result<Vec<T>, ApiError>
    where
        F: Fn(u32, u32) -> Fut,
        Fut: std::future::Future<Output = Result<Vec<T>, ApiError>>,
    {
        let page_size = 100u32;
        let mut all = Vec::new();
        let mut page = 1u32;
        loop {
            let items = fetch(page, page_size).await?;
            let count = items.len();
            all.extend(items);
            if (count as u32) < page_size {
                break;
            }
            page += 1;
        }
        Ok(all)
    }

    pub(crate) async fn handle_empty_response(resp: reqwest::Response) -> Result<(), ApiError> {
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(());
        }
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T, ApiError> {
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            let data = resp.json::<T>().await?;
            return Ok(data);
        }
        if status == 401 {
            return Err(ApiError::Auth(
                "CNB_TOKEN 缺失或无效。请设置：export CNB_TOKEN=\"your_token\"".to_string(),
            ));
        }
        let body = resp.text().await.unwrap_or_default();
        Err(ApiError::HttpStatus { status, body })
    }
}
