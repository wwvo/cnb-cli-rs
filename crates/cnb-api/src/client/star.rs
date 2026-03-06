use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn list_star_users(&self) -> Result<StarUsers, ApiError> {
        let url = format!(
            "{}{}/-/star-users?filter_type=all&page=0&page_size=10000",
            self.base_url, self.repo
        );
        let resp = self.http.get(&url).send().await?;
        Self::handle_response(resp).await
    }
}
