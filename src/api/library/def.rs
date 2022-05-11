use std::collections::HashMap;

use bytes::Bytes;

use crate::api::prelude::*;

use super::{model, payload};

define_request! {
    library,
    get_book_by_id,
    (GET, "/books/:book_id"),
    Path,
    [
        book_id: u32
    ],
    [
        #[error("Not found book")]
        NotFoundBook,
    ],
    [
        StatusCode::NOT_FOUND => |_resp| async {
            Error::NotFoundBook
        }
    ],
    StatusCode::OK => model::Book
}

define_request! {
    library,
    get_books,
    (GET, "/books"),
    Querystring,
    [
        kind: Option<payload::BookKind>,
        per_page: Option<usize>,
        page: Option<usize>,
        sort_by: Option<payload::BookSortBy>,
    ],
    [],
    [],
    StatusCode::OK => Vec<model::Book>
}

define_request! {
    library,
    get_books_by_ids,
    (GET, "/books"),
    Querystring,
    [
        ids: Vec<u32>
    ],
    [],
    [],
    StatusCode::OK => Vec<model::Book>
}

define_request! {
    library,
    get_books_by_tags,
    (GET, "/books"),
    Querystring,
    [
        tags: Vec<(String, String)>,
        per_page: usize,
        page: usize,
        sort_by: Option<payload::BookSortBy>
    ],
    [],
    [],
    StatusCode::OK => HashMap<(String, String), Vec<model::Book>>, // Vec<((String, String), Vec<model::Book>)>
    |buf| {
        let deserialized: Vec<((String, String), Vec<model::Book>)> =
            serde_json::from_slice(&buf).map_err(BaseError::JsonDeserialize)?;

        let r = deserialized.into_iter().collect();

        Ok(r)
    }
}

define_request! {
    library,
    get_book_image_list,
    (GET, "/books/:book_id/images"),
    Querystring,
    [
        book_id: u32
    ],
    [
        #[error("Not found book")]
        NotFoundBook
    ],
    [
        StatusCode::NOT_FOUND => |_resp| async {
            Error::NotFoundBook
        }
    ],
    StatusCode::OK => Vec<String>
}

/*

    Implementation of get book image


    define_request! {
        library,
        get_book_image,
        (GET, "/books/:book_id/images/:file_name"),
        Path,
        [
            book_id: u32,
            file_name: String,
        ],
        [
            #[error("Not found book or image")]
            NotFoundBookOrImage,
        ],
        [
            StatusCode::NOT_FOUND => |_resp| async {
                Error::NotFoundBookOrImage
            }
        ],
        StatusCode::OK => Bytes
    }

*/

#[cfg(feature = "client")]
impl_namespace!(
    library,
    get_book_image,
    [(book_id: u32), (file_name: String)],
    Bytes
);

#[impl_into_args]
pub async fn get_book_image(
    base_url: impl Into<String>,
    token: impl Into<Token<'_>>,
    book_id: u32,
    file_name: impl Into<String>,
) -> Result<Bytes, crate::api::library::error::Error> {
    get_book_image::execute(base_url.into(), token.into(), book_id, file_name.into()).await
}

pub mod get_book_image {
    use bytes::Bytes;
    use http::StatusCode;

    use crate::api::prelude::*;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Not found book or image")]
        NotFoundBookOrImage,
    }

    #[derive(Debug, Serialize)]
    struct Parameter {
        book_id: u32,
        file_name: String,
    }

    pub async fn execute(
        base_url: String,
        token: Token<'_>,
        book_id: u32,
        file_name: String,
    ) -> Result<Bytes, crate::api::library::error::Error> {
        let parameter = Parameter { book_id, file_name };
        log::debug!("path_parameter = {parameter:?}");
        let path = serde_path::to_string("/books/:book_id/images/:file_name", &parameter).unwrap();

        let resp = request(
            GET,
            &base_url,
            &path,
            &token,
            ParameterKind::Path,
            None::<()>,
        )?
        .send()
        .await
        .map_err(BaseError::Reqwest)?;

        response(token, resp, |resp| async {
            match resp.status() {
                StatusCode::OK => {
                    let bytes = resp
                        .bytes()
                        .await
                        .expect("why failed get bytes from response");

                    Ok(bytes)
                }

                StatusCode::NOT_FOUND => Err(Error::NotFoundBookOrImage.into()),

                code => Err(BaseError::from_status(code, resp).await),
            }
        })
        .await
    }
}
