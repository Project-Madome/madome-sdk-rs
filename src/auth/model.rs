use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserId {
    pub user_id: String,
}
