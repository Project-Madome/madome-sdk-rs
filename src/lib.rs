pub mod api;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::MadomeClient;
