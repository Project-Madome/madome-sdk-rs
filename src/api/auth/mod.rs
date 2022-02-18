pub mod def;
pub mod error;
pub mod model;

#[cfg(feature = "server")]
pub const MADOME_PUBLIC_ACCESS_HEADER: &str = "x-madome-public-access";

pub const MADOME_ACCESS_TOKEN: &str = "madome_access_token";
pub const MADOME_REFRESH_TOKEN: &str = "madome_refresh_token";

/* use http::{header, Method, StatusCode};
use reqwest::Client;
use util::http::{Cookie, SetCookie};



pub enum Role {
    Normal,
    Developer,
}

impl Role {
    fn u8(&self) -> u8 {
        use Role::*;

        match self {
            Normal => 0,
            Developer => 1,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Permission denied")]
    PermissionDeniedByRefreshed(SetCookie),

    #[error("Other: {1}")]
    Other(StatusCode, String),

    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl Error {
    #[cfg(feature = "server")]
    pub fn to_http<T>(&self, response: http::response::Builder) -> http::Result<http::Response<T>>
    where
        T: From<String>,
    {
        use util::http::SetHeaders;

        use Error::*;

        match self {
            err @ Unauthorized => response
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into()),

            err @ PermissionDenied => response
                .status(StatusCode::FORBIDDEN)
                .body(err.to_string().into()),

            err @ PermissionDeniedByRefreshed(set_cookie) => response
                .status(StatusCode::FORBIDDEN)
                .headers(set_cookie.iter())
                .body(err.to_string().into()),

            Other(status_code, body) => response.status(status_code).body(body.to_owned().into()),

            err @ Reqwest(_) => response
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string().into()),
        }
    }
}

#[cfg(feature = "server")]
pub fn check_internal(headers: &http::HeaderMap) -> Result<(), Error> {
    let has_public = headers.get(MADOME_PUBLIC_ACCESS_HEADER).is_some();

    if has_public {
        Err(Error::PermissionDenied)
    } else {
        Ok(())
    }
}

pub async fn check_access_token(
    base_url: &str,
    access_token: &str,
    role: Role,
) -> Result<model::UserId, Error> {
    /* Request */
    let url = format!("{}{}", base_url, "/auth/token");

    let mut cookie = Cookie::new();
    cookie.add(MADOME_ACCESS_TOKEN, access_token);

    let mut req = Client::new()
        .request(Method::GET, url)
        .header(header::COOKIE, cookie.to_string());

    if role.u8() > 0 {
        req = req.query(&[("role", role.u8())]);
    }

    let res = req.send().await?;

    /* Response */
    match res.status() {
        StatusCode::OK => {
            let res = res
                .json::<model::UserId>()
                .await
                .expect("response deserialize in auth::check_access_token()");

            Ok(res)
        }

        StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),

        StatusCode::FORBIDDEN => Err(Error::PermissionDenied),

        status_code => Err(Error::Other(
            status_code,
            res.text().await.unwrap_or_default(),
        )),
    }
}

pub async fn create_authcode() {}

pub async fn create_token_pair() {}

pub async fn check_and_refresh_token_pair(
    base_url: &str,
    access_token: &str,
    refresh_token: &str,
    role: Role,
) -> Result<(model::UserId, SetCookie), Error> {
    /* Request */
    let url = format!("{}{}", base_url, "/auth/token");

    let mut cookie = Cookie::new();
    cookie.add(MADOME_ACCESS_TOKEN, access_token);
    cookie.add(MADOME_REFRESH_TOKEN, refresh_token);

    let mut req = Client::new()
        .request(Method::PATCH, url)
        .header(header::COOKIE, cookie.to_string());

    if role.u8() > 0 {
        req = req.query(&[("role", role.u8())]);
    }

    let res = req.send().await?;

    /* Response */
    match res.status() {
        StatusCode::OK => {
            // Maybe has set-cookie headers
            let set_cookie = SetCookie::from_headers(res.headers());

            let res = res
                .json()
                .await
                .expect("response deserialize in auth::check_and_refresh_token_pair()");

            Ok((res, set_cookie))
        }

        StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),

        StatusCode::FORBIDDEN => Err(Error::PermissionDeniedByRefreshed(SetCookie::from_headers(
            res.headers(),
        ))),

        status_code => Err(Error::Other(
            status_code,
            res.text().await.unwrap_or_default(),
        )),
    }
}
 */
