use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_workspaces(
        &self, status: &str, page: i32, page_size: i32,
    ) -> Result<WorkspaceListResponse, ApiError> {
        let mut url = format!(
            "{}user/workspaces?page={page}&page_size={page_size}",
            self.base_url
        );
        if !status.is_empty() {
            url.push_str(&format!("&status={status}"));
        }
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_workspace(&self, pipeline_id: &str) -> Result<(), ApiError> {
        let url = format!("{}user/workspaces/{pipeline_id}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        Self::handle_empty_response(resp).await
    }
}
