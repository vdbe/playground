use axum::{async_trait, body::HttpBody, extract::FromRequest, http::Request, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use serde::{de::DeserializeOwned, Deserialize};
use validator::Validate;

use crate::{
    config::env::JWT_SECRET,
    error::{ApiError, ErrorRepr},
    util::jwt::{ClaimSub, Claims},
};

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Claims<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: ClaimSub + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(request, state)
                .await
                .map_err(ErrorRepr::MissingBearer)?;

        let sub = T::decode(bearer.token(), &JWT_SECRET)?;

        Ok(sub)
    }
}
