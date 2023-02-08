use entity::{
    refresh_token::{
        self as entity_refresh_token, ActiveModel as ActiveModelRefreshToken,
        Entity as EntityRefresToken, Model as ModelRefreshToken,
    },
    user::{self as entity_user, Model as ModelUser},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, QueryFilter,
};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{
    db::error::DbError,
    dto::{auth::RefreshToken, user::User},
    DbConn,
};

use super::error::DbResult;

impl RefreshToken {
    pub(crate) async fn new(
        user_uuid: Uuid,
        expiry_date: PrimitiveDateTime,
        db: &DbConn,
    ) -> DbResult<Self> {
        let active_refresh_token = ActiveModelRefreshToken {
            id: NotSet,
            token: Set(Uuid::new_v4()),
            user_uuid: Set(user_uuid),
            expiry_date: Set(expiry_date),
        };

        let model_refresh_token: ModelRefreshToken =
            active_refresh_token.insert(db).await?;

        Ok(model_refresh_token.into())
    }

    pub(crate) async fn get_user_by_token(
        token: Uuid,
        db: &DbConn,
    ) -> DbResult<(Self, User)> {
        let (model_refresh_token, model_user): (
            ModelRefreshToken,
            Option<ModelUser>,
        ) = EntityRefresToken::find()
            .filter(entity_refresh_token::Column::Token.eq(token))
            .find_also_related(entity_user::Entity)
            .one(db)
            .await?
            .ok_or(DbError::NoResult)?;

        let model_user = model_user.ok_or(DbError::MissingRelation)?;

        Ok((model_refresh_token.into(), model_user.into()))
    }

    pub(crate) async fn drop_by_token(
        token: Uuid,
        db: &DbConn,
    ) -> DbResult<()> {
        let _res = EntityRefresToken::delete_many()
            .filter(entity_refresh_token::Column::Token.eq(token))
            .exec(db)
            .await?;

        Ok(())
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
