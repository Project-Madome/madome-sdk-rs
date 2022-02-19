use crate::api::macros::extend_error;

use super::def;

extend_error![
    #[error("")]
    CreateAuthcode(
        #[from]
        def::create_authcode::Error
    )
];
