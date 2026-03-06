use crate::error::ApiError;
use crate::types::*;
use super::CnbClient;

impl CnbClient {
    pub async fn upload_logo_info(
        &self, group_name: &str, req: &UploadLogoRequest,
    ) -> Result<UploadLogoResponse, ApiError> {
        let group_name = Self::encode_path(group_name);
        let url = format!("{}{}/-/upload/logos", self.base_url, group_name);
        let resp = self.http.post(&url).json(req).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn post_to_cos(
        &self, upload_url: &str, form: &std::collections::HashMap<String, String>, file_data: Vec<u8>,
    ) -> Result<(), ApiError> {
        let mut multipart = reqwest::multipart::Form::new();
        for (key, value) in form {
            multipart = multipart.text(key.clone(), value.clone());
        }
        let part = reqwest::multipart::Part::bytes(file_data).file_name("file.dat");
        multipart = multipart.part("file", part);

        let resp = self.http_plain
            .post(upload_url)
            .multipart(multipart)
            .send()
            .await?;

        let status = resp.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(());
        }
        let text = resp.text().await.unwrap_or_default();
        Err(ApiError::Api(format!("上传失败 HTTP {status}: {text}")))
    }
}
