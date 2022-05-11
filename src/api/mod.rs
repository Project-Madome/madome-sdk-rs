mod error;
mod macros;
mod token;

pub mod auth;
pub mod cookie;
pub mod header;
pub mod library;
pub mod user;

pub use error::BaseError;
pub use token::{Token, TokenBehavior};

#[allow(unused_imports, dead_code)]
pub(crate) mod prelude {
    pub(crate) use reqwest::{Request, Response};
    pub(crate) use serde::{Deserialize, Serialize};

    pub(crate) use madome_sdk_macros::impl_into_args;
    pub(crate) use madome_sdk_macros::ret_ty_or_unit;

    pub(crate) use super::error::BaseError;
    pub(crate) use super::http::ParameterKind;
    pub(crate) use super::http::{request, response};
    pub(crate) use super::macros::*;
    pub(crate) use super::token::Token;

    use http::Method;

    pub(crate) const GET: Method = Method::GET;
    pub(crate) const POST: Method = Method::POST;
    pub(crate) const PUT: Method = Method::PUT;
    pub(crate) const PATCH: Method = Method::PATCH;
    pub(crate) const DELETE: Method = Method::DELETE;
}

mod http {
    use std::future::Future;

    use http::{header, Method};
    use reqwest::{Client, RequestBuilder, Response};
    use serde::Serialize;

    use super::error::BaseError;
    use super::token::Token;

    pub(crate) enum ParameterKind {
        Path,
        Querystring,
        Json,
        Nothing,
    }

    pub(crate) fn request<T>(
        method: Method,
        base_url: &str,
        path: &str,
        token: &Token,
        parameter_kind: ParameterKind,
        parameter: Option<T>,
    ) -> Result<RequestBuilder, BaseError>
    where
        T: Serialize,
    {
        let (cookie, token) = token.as_cookie().into();

        let url = format!("{base_url}{path}");

        let req = Client::new();
        let req = match parameter_kind {
            ParameterKind::Querystring => {
                let qs = serde_qs::to_string(parameter.as_ref().unwrap())
                    .map_err(BaseError::QuerystringSerialize)?;
                log::debug!("serialized_parameter = {qs}");
                req.request(method, format!("{url}?{qs}"))
            }

            ParameterKind::Json => {
                let json = serde_json::to_vec(parameter.as_ref().unwrap())
                    .map_err(BaseError::JsonSerialize)?;
                #[cfg(debug_assertions)]
                {
                    log::debug!(
                        "serialized_parameter = {}",
                        String::from_utf8(json.clone()).unwrap()
                    );
                }
                req.request(method, url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json)
            }

            ParameterKind::Path | ParameterKind::Nothing => req.request(method, url),
        }
        .header(cookie, token);

        #[cfg(test)]
        {
            use super::header::MADOME_E2E_TEST;

            if true {
                return Ok(req.header(MADOME_E2E_TEST, "true"));
            }
        }

        Ok(req)
    }

    #[allow(unused_variables)]
    pub(crate) fn response<T, E, F, Fut>(token: Token, resp: Response, f: F) -> Fut
    where
        F: Fn(Response) -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        token.update(resp.headers());

        f(resp)
    }
}
