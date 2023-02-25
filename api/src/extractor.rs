use axum::{async_trait, extract::FromRequest, http::Request, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{ApiError, ErrorRepr},
    util::jwt::{self, ClaimsDecoded, ClaimsEncoded, ClaimsSubTrait},
};

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ClaimsDecoded<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: ClaimsSubTrait,
    jwt::Decoded<T>: for<'a> Deserialize<'a>,
{
    type Rejection = ApiError;

    async fn from_request(
        request: Request<B>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(request, state)
                .await
                .map_err(ErrorRepr::MissingBearer)?;

        let token = bearer.token();
        let claims: ClaimsEncoded<T> = From::from(token.to_owned());
        let claims = claims.decode()?;

        Ok(claims)
    }
}
