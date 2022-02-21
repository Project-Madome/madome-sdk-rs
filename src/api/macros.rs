macro_rules! extend_error {
    ($($member:tt)*) => {
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("{0}")]
            Base(#[from] $crate::api::error::BaseError),

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
                $fn(self.base_url.clone(), self.token, $($arg_id.into()),*).await
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

        #[impl_into_args]
        pub async fn $fn(base_url: impl Into<String>, token: impl Into<Token<'_>>, $($arg_id: $arg_ty),*) -> Result<$ret_ty, $crate::api::$namespace::error::Error> {
            $fn::execute(base_url.into(), token.into(), $($arg_id.into()),*).await
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
        #[derive(Debug, ::serde::Serialize)]
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
        #[derive(Debug, ::serde::Serialize)]
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

        pub async fn execute(base_url: String, token: Token<'_>, $($arg_id: $arg_ty),*) -> Result<$ret_ty, $crate::api::$namespace::error::Error> {
            let req = match $parameter_kind {
                ParameterKind::Querystring => {
                    let parameter = qs_parameters($($arg_id,)*);
                    ::log::debug!("qs_parameter = {parameter:?}");
                    request($method, &base_url, $path, &token, $parameter_kind, Some(parameter))
                },
                ParameterKind::Json => {
                    let parameter = json_parameters($($arg_id,)*);
                    ::log::debug!("json_parameter = {parameter:?}");
                    request($method, &base_url, $path, &token, $parameter_kind, Some(parameter))
                },
                ParameterKind::Nothing => {
                    request($method, &base_url, $path, &token, $parameter_kind, None::<()>)
                },
            }?;

            let resp = req
                .send()
                .await
                .map_err(BaseError::Reqwest)?;

            response(token, resp, |resp| async {
                match resp.status() {
                    $ok_code => {
                        #[ret_ty_or_unit]
                        #[allow(unused_variables)]
                        async fn deserialize(resp: Response) -> Result<$ret_ty, BaseError> {
                            let buf = resp
                                .bytes()
                                .await
                                .unwrap_or_default();

                            // ::log::debug!("deserializing...");

                            #[cfg(debug_assertions)] {
                                let buf: &[u8] = buf.as_ref();
                                ::log::debug!("{}", String::from_utf8(buf.to_vec()).unwrap());
                            }

                            let deserialized = serde_json::from_slice(&buf)
                                .map_err(BaseError::JsonDeserialize)?;

                            Ok(deserialized)
                        }

                        Ok(deserialize(resp).await?)
                    }

                    $($err_code => {
                        use self::Error;

                        Err(<$crate::api::$namespace::error::Error>::from($err(resp).await))
                    },)*

                    code => Err(BaseError::from_status(code, resp).await),
                }
            })
            .await
        }
    };
}

pub(crate) use define_request;
