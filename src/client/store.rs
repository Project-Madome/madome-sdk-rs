use http::HeaderMap;
use parking_lot::RwLock;
use util::http::{Cookie, SetCookie};

use crate::api::auth::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN};

pub type TokenPair = (String, String);

impl From<String> for AuthStore {
    fn from(access_token: String) -> Self {
        Self {
            token: RwLock::new(Some((access_token, String::new()))),
        }
    }
}

impl From<&str> for AuthStore {
    fn from(access_token: &str) -> Self {
        access_token.to_string().into()
    }
}

impl From<TokenPair> for AuthStore {
    fn from(token: TokenPair) -> Self {
        Self {
            token: RwLock::new(Some(token)),
        }
    }
}

impl From<(&str, &str)> for AuthStore {
    fn from((access_token, refresh_token): (&str, &str)) -> Self {
        (access_token.to_string(), refresh_token.to_string()).into()
    }
}

#[derive(Default)]
pub struct AuthStore {
    token: RwLock<Option<TokenPair>>,
}

impl AuthStore {
    pub fn as_cookie(&self) -> Cookie {
        let (access, refresh) = { self.token.read().clone().unwrap_or_default() };

        let cookie = [
            (MADOME_ACCESS_TOKEN, access),
            (MADOME_REFRESH_TOKEN, refresh),
        ];

        Cookie::from_iter(cookie)
    }

    pub fn update(&self, headers: &HeaderMap) {
        let mut set_cookie = SetCookie::from_headers(headers);

        let access_token = set_cookie.take(MADOME_ACCESS_TOKEN);
        let refresh_token = set_cookie.take(MADOME_REFRESH_TOKEN);

        match (access_token, refresh_token) {
            (Some(access), Some(refresh)) => {
                {
                    let mut token = self.token.write();

                    token.replace((access, refresh));

                    // unlocked here
                }

                log::debug!("token updated = true");
            }
            _ => log::debug!("token updated = false"),
        }
    }
}
