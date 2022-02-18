use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Like {
    Book { book_id: u32 },
    BookTag { tag_kind: String, tag_name: String },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LikeKind {
    Book,
    BookTag,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LikeSortBy {
    CreatedAtDesc,
    CreatedAtAsc,
    Random,
}
