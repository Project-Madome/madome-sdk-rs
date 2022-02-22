use crate::api::prelude::*;

use super::{model, payload};

define_request! {
    old_library,
    get_books,
    (GET, "/v1/books"),
    Querystring,
    [
        r#type: Option<payload::BookType>,
        per_page: usize,
        page: usize,
        sort: Option<payload::BookSortBy>,
    ],
    [],
    [],
    StatusCode::OK => Vec<model::Book>
}

define_request! {
    old_library,
    get_books_by_ids,
    (GET, "/v1/books"),
    Querystring,
    [
        ids: Vec<u32>
    ],
    [],
    [],
    StatusCode::OK => Vec<model::Book>
}
