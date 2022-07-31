use crate::api::macros::extend_error;

use super::def;

extend_error![
    /* #[error("{0}")]
    GetMe(
        #[from]
        def::get_me::Error
    ), */
    #[error("{0}")]
    GetUser(
        #[from]
        def::get_user::Error
    ),
    #[error("{0}")]
    GetLikes(
        #[from]
        def::get_likes::Error
    ),
    #[error("{0}")]
    GetHistories(
        #[from]
        def::get_histories::Error
    ),
    /* #[error("{0}")]
    CreateUser(
        #[from]
        def::create_user::Error
    ), */
    /*
    #[error("{0}")]
    CreateOrUpdateFcmToken(
        #[from]
        def::create_or_update_fcm_token::Error
    ),
    #[error("{0}")]
    CreateLike(
        #[from]
        def::create_like::Error
    ),
    #[error("{0}")]
    DeleteLike(
        #[from]
        def::delete_like::Error
    ) */
];
