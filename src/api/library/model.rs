use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BookKind {
    Manga,
    Doujinshi,
    GameCg,
    ArtistCg,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub tags: Vec<(String, String)>,
    pub kind: BookKind,
    pub language: String,
    pub page: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[test]
#[ignore]
fn test_serialize_book() {
    let now = Utc::now();

    let v = Book {
        id: 123456,
        title: "title".to_string(),
        tags: vec![
            ("female".to_string(), "loli".to_string()),
            ("female".to_string(), "anal".to_string()),
        ],
        kind: BookKind::Doujinshi,
        language: "korean".to_string(),
        page: 34,
        created_at: now,
        updated_at: now,
    };

    let r = serde_json::to_string(&v).unwrap();

    println!("{r}");
}

#[test]
#[ignore]
fn test_serialize_book_in_hashmap() {
    let now = Utc::now();

    let v = Book {
        id: 123456,
        title: "title".to_string(),
        tags: vec![
            ("female".to_string(), "loli".to_string()),
            ("female".to_string(), "anal".to_string()),
        ],
        kind: BookKind::Doujinshi,
        language: "korean".to_string(),
        page: 34,
        created_at: now,
        updated_at: now,
    };

    let map = std::collections::HashMap::<String, Book>::from_iter([("female".to_string(), v)]);

    let r = serde_json::to_string(&map).unwrap();

    println!("{r}");
}
