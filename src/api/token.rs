use util::http::{Cookie, SetCookie};

use crate::api::header::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN};

pub trait TokenBehavior {
    /// use interior-mutability
    fn update(&self, token_pair: (Option<String>, Option<String>));

    fn as_cookie(&self) -> Cookie {
        Default::default()
    }
}

pub enum Token<'a> {
    Origin((String, String)),
    Store(&'a dyn TokenBehavior),
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

    pub fn update(&self, headers: &http::HeaderMap) {
        if let Self::Store(x) = self {
            let mut set_cookie = SetCookie::from_headers(headers);

            let access_token = set_cookie.take(MADOME_ACCESS_TOKEN);
            let refresh_token = set_cookie.take(MADOME_REFRESH_TOKEN);

            x.update((access_token, refresh_token));
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

impl<'a> From<&'a dyn TokenBehavior> for Token<'a> {
    fn from(x: &'a dyn TokenBehavior) -> Self {
        Self::Store(x)
    }
}
