use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_releases(&self, page: u32, page_size: u32) -> Result<Vec<Release>, ApiError> {
        let url = format!("{}{}/-/releases?page={page}&page_size={page_size}",
            self.base_url, self.repo);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn list_all_releases(&self) -> Result<Vec<Release>, ApiError> {
        self.paginate(|page, page_size| self.list_releases(page, page_size)).await
    }

    pub async fn get_release_by_tag(&self, repo_name: &str, tag: &str) -> Result<Release, ApiError> {
        let url = format!("{}{repo_name}/-/releases/tag/{tag}", self.base_url);
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn create_release(&self, req: &CreateReleaseRequest) -> Result<Release, ApiError> {
        let url = format!("{}{}/-/releases", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_release_asset(&self, release_id: &str, asset_id: &str) -> Result<(), ApiError> {
        let url = format!("{}{}/-/releases/{release_id}/assets/{asset_id}", self.base_url, self.repo);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }

    pub async fn get_release_asset_upload_url(
        &self, repo_name: &str, release_id: &str, req: &PostReleaseAssetUploadURLRequest,
    ) -> Result<ReleaseAssetUploadURL, ApiError> {
        let url = format!("{}{repo_name}/-/releases/{release_id}/asset-upload-url", self.base_url);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }
}
