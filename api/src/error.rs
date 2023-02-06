use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error("Interal errror")]
    Db(#[from] sea_orm::DbErr),


    #[error("unknown data store error")]
    Unknown,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub(crate) type ApiError = (StatusCode, Json<Value>);
pub(crate) type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
