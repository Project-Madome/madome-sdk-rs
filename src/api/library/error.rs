use crate::api::macros::extend_error;

use super::def;

extend_error![
    #[error("{0}")]
    GetBookById(
        #[from]
        def::get_book_by_id::Error
    ),
    #[error("{0}")]
    GetBookImageList(
        #[from]
        def::get_book_image_list::Error
    ),
    #[error("{0}")]
    GetBookImage(
        #[from]
        def::get_book_image::Error
    )
];
