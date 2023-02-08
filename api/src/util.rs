use time::{OffsetDateTime, PrimitiveDateTime};
use validator::Validate;

use crate::error::ResultRepr;

pub(crate) mod encryption;
pub(crate) mod jwt;

pub(crate) fn validate_payload<T: Validate>(payload: &T) -> ResultRepr<()> {
    Ok(payload.validate()?)
}

pub fn now_utc() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}
