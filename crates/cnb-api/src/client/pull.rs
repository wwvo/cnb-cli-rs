use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_pulls(&self, opts: &ListPullsOptions) -> Result<Vec<PullRequest>, ApiError> {
        let mut url = format!("{}{}/-/pulls?page={}&page_size={}&state={}",
            self.base_url, self.repo, opts.page, opts.page_size, opts.state);
        if let Some(ref reviewers) = opts.reviewers {
            url.push_str(&format!("&reviewers={reviewers}"));
        }
        if let Some(ref authors) = opts.authors {
            url.push_str(&format!("&authors={authors}"));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn get_head(&self, repo_name: &str) -> Result<HeadRef, ApiError> {
        let url = format!("{}{repo_name}/-/git/head", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn create_pull(&self, repo_name: &str, req: &CreatePullRequest) -> Result<Pull, ApiError> {
        let url = format!("{}{repo_name}/-/pulls", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn update_pull(&self, number: &str, req: &UpdatePullRequest) -> Result<Pull, ApiError> {
        let url = format!("{}{}/-/pulls/{number}", self.base_url, self.repo);
        let resp = self.http.patch(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn merge_pull(&self, number: &str, req: &MergePullRequestBody) -> Result<MergePullResponse, ApiError> {
        let url = format!("{}{}/-/pulls/{number}/merge", self.base_url, self.repo);
        let resp = self.http.put(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
