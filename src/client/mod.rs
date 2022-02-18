pub mod base_url;
pub mod store;

use self::{base_url::MadomeBaseUrl, store::AuthStore};

pub struct MadomeClient {
    base_url: MadomeBaseUrl,
    token: AuthStore,
}

impl MadomeClient {
    pub fn stable() -> Self {
        Self {
            base_url: MadomeBaseUrl::stable(),
            token: Default::default(),
        }
    }

    pub fn beta() -> Self {
        Self {
            base_url: MadomeBaseUrl::beta(),
            token: Default::default(),
        }
    }

    pub fn nightly() -> Self {
        Self {
            base_url: MadomeBaseUrl::nightly(),
            token: Default::default(),
        }
    }
}

macro_rules! impl_madome_client {
    ($($namespace:ident),*$(,)?) => {
        impl MadomeClient {
            $(
                pub fn $namespace(&self) -> $namespace {
                    $namespace {
                        base_url: self.base_url.$namespace.as_str(),
                        token: &self.token
                    }
                }
            )*
        }

        $(
            #[allow(incorrect_ident_case, non_camel_case_types)]
            pub struct $namespace<'a> {
                pub(crate) base_url: &'a str,
                pub(crate) token: &'a AuthStore,
            }
        )*


        /* impl<'a> $namespace<'a> {
            $($(impl_namespace! { $namespace, $fn, [$($arg_id, $arg_ty)*], $ret_ty })*)*
        } */
    };


}

impl_madome_client![user, auth];
