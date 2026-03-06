use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn ai_chat(&self, req: &ChatCompletionsRequest) -> Result<ChatCompletionsResponse, ApiError> {
        let url = format!("{}{}/-/ai/chat/completions", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn ai_chat_stream(&self, req: &ChatCompletionsRequest) -> Result<reqwest::Response, ApiError> {
        let url = format!("{}{}/-/ai/chat/completions", self.base_url, self.repo);
        let resp = self.http.post(&url).json(req).send().await?;
        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(resp);
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
