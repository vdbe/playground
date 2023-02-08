use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref STRING_JWT_REFRESH_SECRET: String =
        env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    static ref STRING_JWT_ACCESS_SECRET: String =
        env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
    pub(crate) static ref JWT_REFRESH_SECRET: &'static [u8] =
        STRING_JWT_REFRESH_SECRET.as_bytes();
    pub(crate) static ref JWT_ACCESS_SECRET: &'static [u8] =
        STRING_JWT_ACCESS_SECRET.as_bytes();
}
