use std::{fmt::Debug, marker::PhantomData, time::Duration};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::ResultRepr;

pub(crate) trait ClaimSub: Sized {
    const DURATION: u64;

    fn claim(self, secret: &[u8]) -> ResultRepr<EncodedClaim<Self>>
    where
        Self: Serialize,
    {
        let encoded_claim = sign(self, Duration::from_secs(Self::DURATION), secret)?;

        Ok(encoded_claim)
    }

    fn decode(claim: &str, secret: &[u8]) -> ResultRepr<Claims<Self>>
    where
        Self: DeserializeOwned,
    {
        Ok(jsonwebtoken::decode(
            claim,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map(|data| data.claims)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims<T: ClaimSub> {
    #[serde(flatten)]
    pub(crate) sub: T,
    exp: i64,
    iat: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct EncodedClaim<T> {
    claim: String,
    #[serde(skip)]
    _type: PhantomData<T>,
}

impl<T: ClaimSub> From<String> for EncodedClaim<T> {
    fn from(value: String) -> Self {
        EncodedClaim {
            claim: value,
            _type: PhantomData,
        }
    }
}

impl<T: ClaimSub> Claims<T> {
    pub(crate) fn new(sub: T, duration: Duration) -> Self {
        let iat = OffsetDateTime::now_utc();
        let exp = iat + duration;

        Self {
            sub,
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
        }
    }
}

fn sign<T: ClaimSub>(sub: T, duration: Duration, secret: &[u8]) -> ResultRepr<EncodedClaim<T>>
where
    T: Serialize,
{
    let encoded_claim = jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(sub, duration),
        &EncodingKey::from_secret(secret),
    )?;

    Ok(EncodedClaim::<T> {
        claim: encoded_claim,
        _type: PhantomData,
    })
}
