use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BookType {
    Manga,
    Doujinshi,
    #[serde(rename = "game cg")]
    GameCg,
    #[serde(rename = "artist cg")]
    ArtistCg,
}

#[derive(Debug, Serialize)]
pub enum BookSortBy {
    #[serde(rename = "created_desc")]
    CreatedAtDesc,
    #[serde(rename = "created_asc")]
    CreatedAtAsc,
    #[serde(rename = "random")]
    Random,
}

#[derive(Debug, Serialize)]
pub enum BookSortByWithoutRandom {
    #[serde(rename = "created_desc")]
    CreatedAtDesc,
    #[serde(rename = "created_asc")]
    CreatedAtAsc,
}
