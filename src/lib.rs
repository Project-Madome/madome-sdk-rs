//! # Madome API
//!
//! ## Auth
//! [AuthAPI]
//!
//! ## User
//! [UserAPI]

pub mod auth;
pub mod user;

/// Auth API
pub use auth::Auth as AuthAPI;
/// User API
pub use user::User as UserAPI;
