use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{
    config::{
        constant::{ACCESS_TOKEN_TIMEOUT, REFRESH_TOKEN_TIMEOUT},
        env::{JWT_ACCESS_SECRET, JWT_REFRESH_SECRET},
    },
    util::jwt::{ClaimsEncoded, ClaimsSubTrait},
};

impl ClaimsSubTrait for SubAccesToken {
    const DURATION: u64 = ACCESS_TOKEN_TIMEOUT;

    fn secret<'a>() -> &'a [u8] {
        &JWT_ACCESS_SECRET
    }
}
impl ClaimsSubTrait for SubRefreshToken {
    const DURATION: u64 = REFRESH_TOKEN_TIMEOUT;

    fn secret<'a>() -> &'a [u8] {
        &JWT_REFRESH_SECRET
    }
}

impl SubAccesToken {
    pub(crate) fn new(user_uuid: Uuid) -> Self {
        Self { uuid: user_uuid }
    }
}

impl SubRefreshToken {
    pub(crate) fn new(token: Uuid) -> Self {
        Self { token }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginPayload {
    pub(crate) refresh_token: ClaimsEncoded<SubRefreshToken>,
    #[serde(flatten)]
    pub(crate) access_token: RefreshPayload,
}

#[derive(Debug, Serialize)]
pub(crate) struct RefreshPayload {
    pub(crate) access_token: ClaimsEncoded<SubAccesToken>,
    pub(crate) token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SubRefreshToken {
    #[serde(rename = "refresh_token")]
    pub(crate) token: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SubAccesToken {
    #[serde(rename = "access_token")]
    pub(crate) uuid: Uuid,
}

#[derive(Debug, Serialize)]
pub(crate) struct RefreshToken {
    #[serde(rename = "refresh_token")]
    pub(crate) token: Uuid,
    #[serde(skip_serializing)]
    pub(crate) user_uuid: Uuid,
    #[serde(skip_serializing)]
    pub(crate) expiry_date: PrimitiveDateTime,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RefreshTokenInput {
    #[serde(rename = "refresh_token")]
    pub(crate) token: ClaimsEncoded<SubRefreshToken>,
}
