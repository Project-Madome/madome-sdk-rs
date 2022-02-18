use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Like {
    Book {
        book_id: u32,
        created_at: DateTime<Utc>,
    },
    BookTag {
        tag_kind: String,
        tag_name: String,
        created_at: DateTime<Utc>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Notification {
    Book {
        book_id: u32,
        book_tags: Vec<(String, String)>,
        created_at: DateTime<Utc>,
    },
}
