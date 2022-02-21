pub mod api;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::MadomeClient;

#[cfg(test)]
mod tests;

/* use madome_sdk_macros::impl_into_args;

#[impl_into_args]
fn abcd(x: impl Into<String>) {}

fn aa() {
    abcd("".to_string())
} */
