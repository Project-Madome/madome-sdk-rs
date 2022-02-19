use http::StatusCode;
use reqwest::Response;

#[derive(Debug, thiserror::Error)]
pub enum BaseError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthoirzed")]
    Unauthorized,

    #[error("Permission Denied")]
    PermissionDenied,

    #[error("Undefined: status_code = {0}; body = {1}")]
    Undefined(StatusCode, String),

    #[error("Json Deserialize: {0}")]
    JsonDeserialize(serde_json::Error),

    #[error("Json Serialize: {0}")]
    JsonSerialize(serde_json::Error),

    #[error("Querystring Sereialize: {0}")]
    QuerystringSerialize(serde_qs::Error),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl BaseError {
    /// match 표현식에서 가장 마지막에 사용해야함
    pub async fn from_status<E>(code: StatusCode, resp: Response) -> E
    where
        E: From<BaseError>,
    {
        match code {
            StatusCode::BAD_REQUEST => {
                let msg = resp.text().await.unwrap_or_default();
                BaseError::BadRequest(msg)
            }
            StatusCode::UNAUTHORIZED => BaseError::Unauthorized,
            StatusCode::FORBIDDEN => BaseError::PermissionDenied,
            code => {
                let body = resp.text().await.unwrap_or_default();
                BaseError::Undefined(code, body)
            }
        }
        .into()
    }
}
