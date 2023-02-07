use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error as ErrorTrait;

pub(crate) use user::UserError;

use self::user::PublicUserError;

mod user;

#[derive(Debug, ErrorTrait)]
pub(crate) enum PublicError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    User(#[from] PublicUserError),

    #[error("internal error")]
    Internal,
}

#[derive(Debug, ErrorTrait)]
pub(crate) enum ErrorRepr {
    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    PasswordHash(#[from] password_hash::errors::Error),

    #[error(transparent)]
    TokioRecv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

pub(crate) type ResultRepr<T> = std::result::Result<T, ErrorRepr>;
pub(crate) type ApiError = (StatusCode, Json<Value>);
pub(crate) type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<ErrorRepr> for PublicError {
    fn from(err: ErrorRepr) -> Self {
        tracing::debug!("ErrorRpr: {:?}", err);

        match err {
            ErrorRepr::Validation(err) => Self::Validation(err),
            ErrorRepr::User(err) => Self::User(err.into()),
            _ => Self::Internal,
        }
    }
}

impl From<ErrorRepr> for ApiError {
    fn from(value: ErrorRepr) -> Self {
        PublicError::from(value).into()
    }
}

impl From<PublicError> for ApiError {
    fn from(err: PublicError) -> Self {
        let status = match err {
            PublicError::Validation(_) => StatusCode::BAD_REQUEST,
            PublicError::User(ref err) => err.into(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
