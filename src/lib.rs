pub mod api;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::MadomeClient;

pub use client::store::{AuthStore, TokenPair};

/* use madome_sdk_macros::impl_into_args;

#[impl_into_args]
fn abcd(x: impl Into<String>) {}

fn aa() {
    abcd("".to_string())
} */

/* use madome_sdk_macros::ret_ty_or_unit;

#[ret_ty_or_unit]
async fn efg() -> bytes::Bytes {
    Default::default()
} */
