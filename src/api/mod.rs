mod error;
mod macros;
mod token;

pub mod auth;
pub mod user;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use reqwest::{Request, Response};
    pub(crate) use serde::{Deserialize, Serialize};

    #[cfg(feature = "client")]
    pub(crate) use madome_sdk_macros::impl_into_args;

    pub(crate) use super::error::BaseError;
    pub(crate) use super::macros::*;
    pub(crate) use super::token::Token;
    pub(crate) use super::ParameterKind;
    pub(crate) use super::{request, response};

    use http::Method;

    pub(crate) const GET: Method = Method::GET;
    pub(crate) const POST: Method = Method::POST;
    pub(crate) const PATCH: Method = Method::PATCH;
    pub(crate) const DELETE: Method = Method::DELETE;
}

use std::future::Future;

use http::Method;
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;

use error::BaseError;
use token::Token;

pub(crate) enum ParameterKind {
    Querystring,
    Json,
    Nothing,
}

pub(crate) fn request<T>(
    method: Method,
    base_url: &str,
    url: &str,
    token: &Token,
    parameter_kind: ParameterKind,
    parameter: Option<T>,
) -> Result<RequestBuilder, BaseError>
where
    T: Serialize,
{
    let (key, value) = token.as_cookie().into();

    let url = format!("{base_url}/{url}");

    let req = Client::new();
    let req = match parameter_kind {
        ParameterKind::Querystring => {
            let qs = serde_qs::to_string(parameter.as_ref().unwrap())
                .map_err(BaseError::QuerystringSerialize)?;
            req.request(method, format!("{url}?{qs}"))
        }
        ParameterKind::Json => {
            let json = serde_json::to_vec(parameter.as_ref().unwrap())
                .map_err(BaseError::JsonSerialize)?;
            req.request(method, url).body(json)
        }
        ParameterKind::Nothing => req.request(method, url),
    }
    .header(key, value);

    Ok(req)
}

#[allow(unused_variables)]
pub(crate) fn response<T, E, F, Fut>(token: Token, resp: Response, f: F) -> Fut
where
    F: Fn(Response) -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    #[cfg(feature = "client")]
    token.update(resp.headers());

    f(resp)
}
