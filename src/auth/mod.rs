pub mod model;

use http::{header, Method, StatusCode};
use reqwest::Client;
use util::http::{Cookie, SetCookie};

#[cfg(feature = "server")]
pub const MADOME_PUBLIC_ACCESS_HEADER: &str = "x-madome-public-access";

pub const MADOME_ACCESS_TOKEN: &str = "madome_access_token";
pub const MADOME_REFRESH_TOKEN: &str = "madome_refresh_token";

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

pub struct Auth<'a> {
    base_url: &'a str,
}

impl<'a> Auth<'a> {
    #[cfg(feature = "server")]
    pub fn check_internal(&self, headers: &http::HeaderMap) -> Result<(), Error> {
        let has_public = headers.get(MADOME_PUBLIC_ACCESS_HEADER).is_some();

        if has_public {
            Err(Error::PermissionDenied)
        } else {
            Ok(())
        }
    }

    pub fn new(base_url: &'a str) -> Self {
        Self { base_url }
    }

    /// # Check Access Token
    ///
    /// `Access Token`이 유효한지 확인하고, 권한도 확인합니다.
    ///
    /// * Url
    ///
    /// `/auth/token`
    ///
    /// * Method
    ///
    /// `GET`
    ///
    /// * Query Parameters
    ///
    ///     * Optional
    ///         * `role`: 0 ~ 1
    ///
    /// * Success Response
    ///
    ///     user_id를 리턴함
    ///     * StatusCode: `200`
    ///     * Content-Type: `application/json`
    ///     * Content: `{ "user_id": "1e441e4d-f065-4f30-8c59-7e725f18ecf0" }`
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `401`
    ///         * Reason: `Access Token`이 유효하지 않습니다. 만료되었을 수도 있습니다
    ///
    ///     * StatusCode: `403`
    ///         * Reason: 해당 유저는 권한이 부족합니다. 또는 본인의 것이 아닌 정보에 접근 했을 수도 있습니다.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X GET \
    ///     -H "Cookie: madome_access_token=ACCESS_TOKEN" \
    ///     /auth/token
    ///     ```
    pub async fn check_access_token(
        &self,
        access_token: &str,
        role: Role,
    ) -> Result<model::UserId, Error> {
        /* Request */
        let url = format!("{}{}", self.base_url, "/auth/token");

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

    /// # Create Authcode
    ///
    /// Authcode를 생성하고 이메일을 보냅니다.
    ///
    /// * Url
    ///
    /// `/auth/code`
    ///
    /// * Method
    ///
    /// `POST`
    ///
    /// * Body Parameters
    ///
    ///     * Content-Type: `application/json`
    ///     * Content:
    ///         ```json
    ///         { "email": "user@madome.app" }
    ///         ```
    ///
    /// * Success Response
    ///
    ///     * StatusCode: `201`
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `404`
    ///         * Reason: 해당 유저는 존재하지 않습니다.
    ///
    ///     * StatusCode: `429`
    ///         * Reason: 너무 많이 요청했습니다. 나중에 다시 시도해주세요.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X POST \
    ///     -H "Content-Type: application/json"
    ///     -d '{ "email": "user@madome.app" }' \
    ///     /auth/code
    ///     ```
    pub async fn create_authcode() {}

    /// # Create Token Pair
    ///
    /// `authcode` 인증 후, `Access Token`과 `Refresh Token`을 생성합니다.
    ///
    /// * Url
    ///
    /// `/auth/token`
    ///
    /// * Method
    ///
    /// `POST`
    ///
    /// * Body Parameters
    ///
    ///     * Content-Type: `application/json`
    ///     * Content:
    ///         ```json
    ///         { "email": "user@madome.app"
    ///         , "code": "vrKwuatEgq-V" }
    ///         ```
    ///
    /// * Success Response
    ///
    ///     * StatusCode: `201`
    ///     * Set-Cookie
    ///         * madome_access_token=ACCESS_TOKEN
    ///         * madome_refresh_token=REFRESH_TOKEN
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `401`
    ///         * Reason: `Pair`가 아니거나, `Access Token` 또는 `Refresh Token`이 유효하지 않거나, `Refresh Token`이 만료되었을 수도 있습니다.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X POST \
    ///     -H "Content-Type: application/json"
    ///     -d '{ "email": "user@madome.app", "code": "vrKwuatEgq-V" }' \
    ///     /auth/token
    ///     ```
    pub async fn create_token_pair() {}

    /// # Check And Refresh Token Pair
    ///
    /// `Access Token`을 먼저 인증하고, 만료되었으면
    /// 만료된 `Access Token`과 만료되지 않은 `Refresh Token`을 이용해 새로운 `Access Token`과 `Refresh Token`을 생성합니다.
    ///
    /// 생성하는데 이용된 `Access Token`과 `Refresh Token`은 폐기됩니다.
    ///
    /// 인증 -> 재발급 -> 인증
    ///
    /// * Url
    ///
    /// `/auth/token`
    ///
    /// * Method
    ///
    /// `PATCH`
    ///
    /// * Cookie Parameters
    ///     * madome_access_token=ACCESS_TOKEN
    ///     * madome_refresh_token=REFRESH_TOKEN
    ///
    /// * Success Response
    ///
    ///     * StatusCode: `200`
    ///     * Set-Cookie
    ///         * madome_access_token=ACCESS_TOKEN
    ///         * madome_refresh_token=REFRESH_TOKEN
    ///     * Content-Type: `application/json`
    ///     * Content: `{ "user_id": "1e441e4d-f065-4f30-8c59-7e725f18ecf0" }`
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `401`
    ///         * Reason: `Access Token`이 유효하지 않거나. `Refresh Token`이 만료되었을 수도 있습니다
    ///
    ///     * StatusCode: `403`
    ///         * Description: `Access Token`이 만료된 상태에서 `Check And Refresh Token Pair`에 요청을 보낸 경우,
    ///             `Access Token`을 인증하는 과정에서 권한 확인이 불가능하므로,
    ///             `Refreshed Access Token`으로 권한 확인을 합니다.
    ///             이때 권한이 부족하면 `Permission Denied` 에러를 반환하는데,
    ///             해당 에러와 함께 `Refreshed Token Pair`를 아래에 명시된 것과 같이 `Set-Cookie` 헤더에 담아서 전달하게 됩니다.
    ///             첫 인증 과정에서 `Permission Denied` 에러가 발생한 경우에는 `Access Token`이 만료되지 않았다는 뜻이므로 `Refresh`를 하지 않습니다.
    ///         * Set-Cookie
    ///             * madome_access_token=ACCESS_TOKEN
    ///             * madome_refresh_token=REFRESH_TOKEN
    ///         * Reason: 해당 유저는 권한이 부족합니다. 또는 본인의 것이 아닌 정보에 접근 했을 수도 있습니다.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X PATCH \
    ///     -H "Cookie: madome_access_token=ACCESS_TOKEN; madome_refresh_token=REFRESH_TOKEN"
    ///     /auth/token
    ///     ```
    pub async fn check_and_refresh_token_pair(
        &self,
        access_token: &str,
        refresh_token: &str,
        role: Role,
    ) -> Result<(model::UserId, SetCookie), Error> {
        /* Request */
        let url = format!("{}{}", self.base_url, "/auth/token");

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

            StatusCode::FORBIDDEN => Err(Error::PermissionDeniedByRefreshed(
                SetCookie::from_headers(res.headers()),
            )),

            status_code => Err(Error::Other(
                status_code,
                res.text().await.unwrap_or_default(),
            )),
        }
    }
}
