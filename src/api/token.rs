use util::http::Cookie;

use crate::api::header::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN};

pub enum Token<'a> {
    Origin((String, String)),
    #[cfg(feature = "client")]
    Store(&'a crate::client::store::AuthStore),

    Holder(&'a str),
}

impl Token<'_> {
    pub fn as_cookie(&self) -> Cookie {
        match self {
            Self::Origin((access, refresh)) => Cookie::from_iter([
                (MADOME_ACCESS_TOKEN, access.as_str()),
                (MADOME_REFRESH_TOKEN, refresh.as_str()),
            ]),
            #[cfg(feature = "client")]
            Self::Store(x) => x.as_cookie(),
            Self::Holder(_) => unreachable!(),
        }
    }

    #[cfg(feature = "client")]
    pub fn update(&self, headers: &http::HeaderMap) {
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

#[cfg(feature = "client")]
impl<'a> From<&'a crate::client::store::AuthStore> for Token<'a> {
    fn from(x: &'a crate::client::store::AuthStore) -> Self {
        Self::Store(x)
    }
}
