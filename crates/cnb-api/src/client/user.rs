//! 用户相关 API

use super::CnbClient;
use crate::error::ApiError;
use crate::types::{ActivityDate, UserFollowResult};
use std::fmt::Write;
use urlencoding::encode;

impl CnbClient {
    /// 获取粉丝列表
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_followers(
        &self,
        username: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserFollowResult>, ApiError> {
        let url = format!(
            "{}/users/{}/followers?page={page}&page_size={page_size}",
            self.base_url,
            encode(username)
        );
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取关注列表
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_following(
        &self,
        username: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<UserFollowResult>, ApiError> {
        let url = format!(
            "{}/users/{}/following?page={page}&page_size={page_size}",
            self.base_url,
            encode(username)
        );
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取活动汇总
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_activities(
        &self,
        username: &str,
        date: Option<&str>,
    ) -> Result<ActivityDate, ApiError> {
        let mut url = format!("{}/users/{}/activities", self.base_url, encode(username));
        if let Some(d) = date {
            let _ = write!(url, "?date={}", encode(d));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    /// 获取仓库活动详情
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_repo_activity_details(
        &self,
        username: &str,
        activity_type: &str,
        slug: &str,
        date: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let url = format!(
            "{}/users/{}/repo-activities/{}?slug={}&date={}",
            self.base_url,
            encode(username),
            encode(activity_type),
            encode(slug),
            encode(date),
        );
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }
}
