use axum::http::StatusCode;
use thiserror::Error as ErrorTrait;

#[derive(Debug, ErrorTrait)]
pub(crate) enum PublicUserError {
    #[error("invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, ErrorTrait)]
pub(crate) enum UserError {
    #[error("password required")]
    PasswordRequired,

    #[error("password wrong")]
    PasswordWrong,

    #[error("user has no password")]
    NoPassword,

    #[error("user not found")]
    NotFound,
}

impl From<UserError> for PublicUserError {
    fn from(_err: UserError) -> Self {
        Self::InvalidCredentials
    }
}

impl From<&PublicUserError> for StatusCode {
    fn from(err: &PublicUserError) -> Self {
        match err {
            PublicUserError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }
}
