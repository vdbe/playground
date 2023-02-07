use time::{OffsetDateTime, PrimitiveDateTime};
use validator::Validate;

use crate::error::ResultRepr;

pub(crate) mod encryption;
pub(crate) mod jwt;

pub(crate) fn validate_payload<T: Validate>(payload: &T) -> ResultRepr<()> {
    Ok(payload.validate()?)
}

pub(crate) fn into_option_vec<T, U>(vec: Vec<T>) -> Option<Vec<U>>
where
    T: Into<U>,
{
    if vec.is_empty() {
        return None;
    }

    let vec = vec.into_iter().map(Into::into).collect();

    Some(vec)
}

pub fn now_utc() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}
