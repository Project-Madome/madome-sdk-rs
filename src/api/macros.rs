macro_rules! extend_error {
    ($($member:tt)*) => {
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("{0}")]
            Base(#[from] $crate::api::BaseError),

            $(
                $member
            )*
        }
    };
}

pub(crate) use extend_error;

#[cfg(feature = "client")]
macro_rules! impl_namespace {
    ($namespace:ident, $fn:ident, [$(($arg_id:ident: $arg_ty:ty)),*$(,)?], $ret_ty:ty) => {
        impl $crate::client::$namespace<'_> {
            #[impl_into_args]
            pub async fn $fn(self, $($arg_id: $arg_ty),*) -> Result<$ret_ty, $crate::api::$namespace::error::Error> {
                $fn(&self.base_url, self.token, $($arg_id.into()),*).await
            }
        }
    };
}

#[cfg(feature = "client")]
pub(crate) use impl_namespace;

macro_rules! define_request {
    ($namespace:ident,
    $fn:ident,
    ($method:expr, $path:expr),
    $parameter_kind:expr,
    [$($arg_id:ident: $arg_ty:ty),*$(,)?],
    [$($err_member:tt)*],
    [$($err_code:path => $err:expr),*$(,)?],
    $ok_code:path => $ret_ty:ty) =>
        {
        #[cfg(feature = "client")]
        impl_namespace!($namespace, $fn, [$(($arg_id: $arg_ty)),*], $ret_ty);

        pub async fn $fn(base_url: &str, token: impl Into<Token<'_>>, $($arg_id: $arg_ty),*) -> Result<$ret_ty, $crate::api::$namespace::error::Error> {
            $fn::execute(base_url, token, $($arg_id),*).await
        }

        pub mod $fn {
            use http::StatusCode;

            #[allow(unused_imports)]
            use $crate::api::$namespace::model::*;
            use $crate::api::$namespace::def::*;
            // use $crate::api::macros::*;

            use ParameterKind::*;

            #[derive(Debug, thiserror::Error)]
            pub enum Error {
                $($err_member)*
            }

            define_request!(@def_fn $namespace, $method, $path, $parameter_kind, [$(($arg_id, $arg_ty)),*], [$($err_code => $err),*], $ok_code, $ret_ty);
        }
    };

    (@def_qs [$($arg_id:ident, $arg_ty:ty),*$(,)?]) => {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "kebab-case")]
        pub(crate) struct QuerystringParameters {
            $(
                $arg_id: $arg_ty,
            )*
        }

        pub(crate) fn qs_parameters($($arg_id: $arg_ty),*) -> QuerystringParameters {
            QuerystringParameters {
                $($arg_id,)*
            }
        }
    };

    (@def_json [$($arg_id:ident, $arg_ty:ty),*$(,)?]) => {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct JsonParameters {
            $(
                $arg_id: $arg_ty,
            )*
        }

        pub(crate) fn json_parameters($($arg_id: $arg_ty),*) -> JsonParameters {
            JsonParameters {
                $($arg_id,)*
            }
        }
    };

    (@def_fn
        $namespace:ident,
        $method:expr,
        $path:expr,
        $parameter_kind:expr,
        [$(($arg_id:ident, $arg_ty:ty)),*$(,)?],
        [$($err_code:path => $err:expr),*$(,)?],
        $ok_code:path,
        $ret_ty:ty) => {
        define_request!(@def_qs [$($arg_id, $arg_ty),*]);
        define_request!(@def_json [$($arg_id, $arg_ty),*]);

        pub async fn execute(base_url: &str, token: impl Into<Token<'_>>, $($arg_id: $arg_ty),*) -> Result<$ret_ty, $crate::api::$namespace::error::Error> {
            let token = token.into();

            let req = match $parameter_kind {
                ParameterKind::Querystring => {
                    let parameter = qs_parameters($($arg_id,)*);
                    request($method, base_url, $path, &token, $parameter_kind, Some(parameter))
                },
                ParameterKind::Json => {
                    let parameter = json_parameters($($arg_id,)*);
                    request($method, base_url, $path, &token, $parameter_kind, Some(parameter))
                },
                ParameterKind::Nothing => {
                    request($method, base_url, $path, &token, $parameter_kind, None::<()>)
                },
            }?;

            let resp = req
                .send()
                .await
                .map_err(BaseError::Reqwest)?;

            response(token, resp, |resp| async {
                match resp.status() {
                    $ok_code => {
                        define_request!(@def_resp resp, $ret_ty)
                    }

                    $($err_code => {
                        use self::Error;

                        Err(<$crate::api::$namespace::Error>::from($err(resp).await))
                    },)*

                    code => Err(BaseError::from_status(code, resp).await),
                }
            })
            .await
        }
    };

    (@def_resp $resp:expr, ()) => {
        { Ok(()) }
    };

    (@def_resp $resp:expr, $ret_ty:ty) => {
        {
            let buf = $resp.bytes().await.unwrap_or_default();
            let deserialized = serde_json::from_slice(&buf)
                .map_err(BaseError::JsonDeserialize)?;

            Ok(deserialized)
        }
    };
}

pub(crate) use define_request;
