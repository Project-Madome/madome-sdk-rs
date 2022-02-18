use uuid::Uuid;

use crate::api::prelude::*;

use super::{model, payload};

define_request! {
    user,
    create_user,
    (POST, "/users"),
    Json,
    [name: String, email: String, role: Option<u8>],
    [
        #[error("Bad Request: {0}")]
        BadRequest(String),
        #[error("Conflict")]
        Conflict,
    ],
    [
        StatusCode::BAD_REQUEST => |resp: Response| async {
            let body = resp.text().await.unwrap_or_default();
            Error::BadRequest(body)
        },
        StatusCode::CONFLICT => |_resp: Response| async {
            Error::Conflict
        }
    ],
    StatusCode::OK => ()
}

define_request! {
    user,
    get_me,
    (GET, "/users/@me"),
    Nothing,
    [],
    [], // 404 에러가 있지만 애초에 404에러가 발생하기 전에 인증 에러가 발생함
    [], // NotFound에러는 핸들링 해줄 이유가 없음
    StatusCode::OK => model::User
}

define_request! {
    user,
    create_or_update_fcm_token,
    (PATCH, "/users/@me/fcm-token"),
    Json,
    [udid: Uuid, fcm_token: String],
    [],
    [],
    StatusCode::CREATED => ()
}

define_request! {
    user,
    get_likes,
    (GET, "/users/@me/likes"),
    Querystring,
    [
        kind: Option<payload::LikeKind>,
        per_page: usize, page: usize,
        sort_by: Option<payload::LikeSortBy>],
    [
        #[error("Not Found")]
        NotFound,
    ],
    [
        StatusCode::BAD_REQUEST => |_resp: Response| async {
            Error::NotFound
        }
    ],
    StatusCode::OK => Vec<model::Like>
}

define_request! {
    user,
    create_like,
    (POST, "/users/@me/likes"),
    Json,
    [like: payload::Like],
    [
        #[error("Already Exists Like")]
        AlreadyExistsLike
    ],
    [
        StatusCode::CONFLICT => |_resp: Response| async {
            Error::AlreadyExistsLike
        }
    ],
    StatusCode::CREATED => ()
}

define_request! {
    user,
    delete_like,
    (DELETE, "/users/@me/likes"),
    Json,
    [like: payload::Like],
    [
        #[error("Not Found Like")]
        NotFoundLike
    ],
    [
        StatusCode::NOT_FOUND => |_resp: Response| async {
            Error::NotFoundLike
        }
    ],
    StatusCode::NO_CONTENT => ()
}

define_request! {
    user,
    get_notifications,
    (GET, "/users/@me/notifications"),
    Nothing,
    [],
    [],
    [],
    StatusCode::OK => Vec<model::Notification>
}
