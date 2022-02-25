use util::http::{Cookie, SetCookie};

use crate::api::cookie::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN};

pub trait TokenBehavior: Send + Sync {
    /// use interior-mutability
    fn update(&self, token_pair: (Option<String>, Option<String>));

    /// 요청 보낼때 헤더로 변환할 때 씀
    /// `let (header_key, header_value) = t.as_cookie().into();`
    fn as_cookie(&self) -> Cookie {
        Default::default()
    }
}

#[derive(Clone)]
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

#[cfg(feature = "server")]
mod server {
    use hyper::Body;
    use parking_lot::RwLock;

    use http::Response;

    use util::http::{Cookie, SetResponse};

    use crate::api::cookie::{MADOME_ACCESS_TOKEN, MADOME_REFRESH_TOKEN};

    use super::{Token, TokenBehavior};

    // server 쪽에서 수동으로 유저 토큰을 push 해줘야함
    impl TokenBehavior for RwLock<&mut Response<Body>> {
        fn update(&self, token_pair: (Option<String>, Option<String>)) {
            match token_pair {
                (Some(access_token), Some(refresh_token)) => {
                    {
                        let mut resp = self.write();

                        resp.set_header(MADOME_ACCESS_TOKEN, access_token).unwrap();
                        resp.set_header(MADOME_REFRESH_TOKEN, refresh_token)
                            .unwrap();

                        // droped here
                    }
                    log::debug!("token updated = true");
                }
                _ => log::debug!("token updated = false"),
            }
        }

        fn as_cookie(&self) -> Cookie {
            let resp = self.read();
            Cookie::from(resp.headers())
        }
    }

    impl<'a> From<&'a RwLock<&mut Response<Body>>> for Token<'a> {
        fn from(x: &'a RwLock<&mut Response<Body>>) -> Self {
            Self::Store(x)
        }
    }
}
