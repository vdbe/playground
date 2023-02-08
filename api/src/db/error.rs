pub(crate) type DbResult<T> = std::result::Result<T, DbError>;

use thiserror::Error as ErrorTrait;

#[derive(Debug, ErrorTrait)]
pub(crate) enum DbError {
    #[error("no result")]
    NoResult,

    #[error("missing relation")]
    MissingRelation,
}

impl From<sea_orm::DbErr> for DbError {
    fn from(err: sea_orm::DbErr) -> Self {
        {
            dbg!(err);
            todo!();
        }
    }
}
