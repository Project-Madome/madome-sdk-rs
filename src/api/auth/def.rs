use crate::api::prelude::*;

use super::model::UserId;

pub fn check_internal(headers: &http::HeaderMap) -> Result<(), crate::api::auth::error::Error> {
    let has_public = headers
        .get(crate::api::header::MADOME_PUBLIC_ACCESS_HEADER)
        .is_some();

    if has_public {
        Err(BaseError::PermissionDenied.into())
    } else {
        Ok(())
    }
}

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
    [
        StatusCode::NOT_FOUND => |_resp: Response| async {
            Error::NotFoundUser
        },
        StatusCode::TOO_MANY_REQUESTS => |_resp: Response| async {
            Error::TooManyCreatedAuthcode
        },
    ],
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

define_request! {
    auth,
    refresh_token_pair,
    (PATCH, "/auth/token"),
    Nothing,
    [],
    [],
    [],
    StatusCode::OK => ()
}
