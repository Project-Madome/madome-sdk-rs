use http::HeaderMap;
use util::http::Cookie;

use crate::{
    api::auth::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN},
    client::store::AuthStore,
};

pub enum Token<'a> {
    Origin((String, String)),
    Store(&'a AuthStore),
}

impl Token<'_> {
    pub fn as_cookie(&self) -> Cookie {
        match self {
            Self::Origin((access, refresh)) => Cookie::from_iter([
                (MADOME_ACCESS_TOKEN, access.as_str()),
                (MADOME_REFRESH_TOKEN, refresh.as_str()),
            ]),
            Self::Store(x) => x.as_cookie(),
        }
    }

    pub fn update(&self, headers: &HeaderMap) {
        if let Self::Store(x) = self {
            x.update(headers);
        }
    }
}

impl From<String> for Token<'_> {
    fn from(access: String) -> Self {
        Self::Origin((access, String::new()))
    }
}

impl From<&'_ str> for Token<'_> {
    fn from(x: &'_ str) -> Self {
        x.to_string().into()
    }
}

impl From<(String, String)> for Token<'_> {
    fn from(token: (String, String)) -> Self {
        Self::Origin(token)
    }
}

impl From<(&'_ str, &'_ str)> for Token<'_> {
    fn from(x: (&'_ str, &'_ str)) -> Self {
        (x.0.to_string(), x.1.to_string()).into()
    }
}

impl<'a> From<&'a AuthStore> for Token<'a> {
    fn from(x: &'a AuthStore) -> Self {
        Self::Store(x)
    }
}
