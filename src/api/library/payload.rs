use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BookKind {
    Manga,
    Doujinshi,
    GameCg,
    ArtistCg,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BookSortBy {
    IdDesc,
    IdAsc,
    Random,
}

/* #[derive(Debug, Serialize)]
pub enum BookSortByWithoutRandom {
    #[serde(rename = "created_desc")]
    CreatedAtDesc,
    #[serde(rename = "created_asc")]
    CreatedAtAsc,
} */
