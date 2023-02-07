use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref STRING_JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    pub(crate) static ref JWT_SECRET: &'static [u8] = STRING_JWT_SECRET.as_bytes();
}
