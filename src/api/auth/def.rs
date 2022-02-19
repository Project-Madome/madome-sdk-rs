use crate::api::prelude::*;

use super::model::UserId;

define_request! {
    auth,
    create_authcode,
    (POST, "/auth/code"),
    Json,
    [email: String],
    [
        #[error("Not Found User")]
        NotFoundUser,
        #[error("Too Many Created Authcode")]
        TooManyCreatedAuthcode
    ],
    [],
    StatusCode::CREATED => ()
}

define_request! {
    auth,
    create_token_pair,
    (POST, "/auth/token"),
    Json,
    [email: String, code: String],
    [
        #[error("Not Found Authcode or User")]
        NotFoundAuthcodeOrUser
    ],
    [],
    StatusCode::CREATED => ()
}

define_request! {
    auth,
    check_access_token,
    (GET, "/auth/token"),
    Querystring,
    [role: Option<u8>],
    [],
    [],
    StatusCode::OK => UserId
}
