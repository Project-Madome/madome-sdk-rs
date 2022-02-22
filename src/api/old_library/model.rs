use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookType {
    Manga,
    Doujinshi,
    #[serde(rename = "game cg")]
    GameCg,
    #[serde(rename = "artist cg")]
    ArtistCg,
}

#[derive(Debug, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub group: Vec<String>,
    pub characters: Vec<String>,
    pub artists: Vec<String>,
    pub series: Vec<String>,
    pub tags: Vec<String>,
    pub r#type: BookType,
    pub language: String,
    pub page_count: usize,
    pub created_at: DateTime<Utc>,
}
