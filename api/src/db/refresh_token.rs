use entity::refresh_token::{
    self as entity_refresh_token, ActiveModel as ActiveModelRefreshToken,
    Model as ModelRefreshToken,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{
    dto::user::{RefreshToken, User},
    error::ResultRepr,
    DbConn,
};

impl RefreshToken {
    pub(crate) async fn new(
        user: &User,
        expiry_date: PrimitiveDateTime,
        db: &DbConn,
    ) -> ResultRepr<Self> {
        let active_refresh_token = ActiveModelRefreshToken {
            id: NotSet,
            token: Set(Uuid::new_v4()),
            user_uuid: Set(user.uuid),
            expiry_date: Set(expiry_date),
        };

        let model_user: ModelRefreshToken = active_refresh_token.insert(db).await?;

        Ok(model_user.into())
    }
}

impl From<ModelRefreshToken> for RefreshToken {
    fn from(value: ModelRefreshToken) -> Self {
        Self {
            token: value.token,
            user_uuid: value.user_uuid,
            expiry_date: value.expiry_date,
        }
    }
}
