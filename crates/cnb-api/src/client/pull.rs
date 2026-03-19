use super::CnbClient;
use crate::error::ApiError;
use crate::types::{
    CreatePullRequest, HeadRef, ListPullsOptions, MergePullRequestBody, MergePullResponse, Pull,
    PullRequest, UpdatePullRequest,
};
use std::fmt::Write;
use urlencoding::encode;

impl CnbClient {
    /// 列出 Pull Request。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn list_pulls(&self, opts: &ListPullsOptions) -> Result<Vec<PullRequest>, ApiError> {
        let mut url = format!(
            "{}{}/-/pulls?page={}&page_size={}&state={}",
            self.base_url,
            self.repo,
            opts.page,
            opts.page_size,
            encode(&opts.state)
        );
        if let Some(ref reviewers) = opts.reviewers {
            let _ = write!(url, "&reviewers={}", encode(reviewers));
        }
        if let Some(ref authors) = opts.authors {
            let _ = write!(url, "&authors={}", encode(authors));
        }
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 列出全部 Pull Request（自动分页）。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if any paginated request fails, a response cannot be
    /// deserialized, or the CNB API returns a non-success status.
    pub async fn list_all_pulls(
        &self,
        opts: &ListPullsOptions,
    ) -> Result<Vec<PullRequest>, ApiError> {
        self.paginate(|page, page_size| {
            let mut opts = opts.clone();
            async move {
                opts.page = page;
                opts.page_size = page_size;
                self.list_pulls(&opts).await
            }
        })
        .await
    }

    /// 获取仓库当前 HEAD 引用。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn get_head(&self, repo_name: &str) -> Result<HeadRef, ApiError> {
        let repo_name = Self::encode_path(repo_name);
        let url = format!("{}{repo_name}/-/git/head", self.base_url);
        let resp = self.send_with_retry(|| self.http.get(&url)).await?;
        Self::handle_response(resp).await
    }

    /// 创建 Pull Request。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn create_pull(
        &self,
        repo_name: &str,
        req: &CreatePullRequest,
    ) -> Result<Pull, ApiError> {
        let repo_name = Self::encode_path(repo_name);
        let url = format!("{}{repo_name}/-/pulls", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 更新 Pull Request。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn update_pull(
        &self,
        number: &str,
        req: &UpdatePullRequest,
    ) -> Result<Pull, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    /// 合并 Pull Request。
    ///
    /// # Errors
    ///
    /// Returns [`ApiError`] if the request fails, the response cannot be deserialized,
    /// or the CNB API returns a non-success status.
    pub async fn merge_pull(
        &self,
        number: &str,
        req: &MergePullRequestBody,
    ) -> Result<MergePullResponse, ApiError> {
        let number = encode(number);
        let url = format!("{}{}/-/pulls/{number}/merge", self.base_url, self.repo);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
