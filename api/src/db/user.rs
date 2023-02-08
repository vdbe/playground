use entity::user::{
    self as entity_user, ActiveModel as ActiveModelUser, Entity as EntityUser,
    Model as ModelUser,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    QuerySelect, Unchanged,
};
use uuid::Uuid;

use crate::{dto::user::User, util::now_utc, DbConn};

use super::error::{DbError, DbResult};

// TODO: Create `UserIdentifier` enum with the variants id/uuid/email
// and use this instead of seperate identiefiers
impl User {
    pub(crate) async fn create(self, db: &DbConn) -> DbResult<Self> {
        let active_model_user = ActiveModelUser {
            uuid: Set(self.uuid),
            displayname: Set(self.display_name),
            email: Set(self.email),
            password: Set(self.password),
            created_at: Set(self.created_at),
            updated_at: Set(self.updated_at),
            ..Default::default()
        };

        let model_user = active_model_user.insert(db).await?;

        Ok(model_user.into())
    }

    pub(crate) async fn get_by_uuid(uuid: Uuid, db: &DbConn) -> DbResult<Self> {
        let user: User = EntityUser::find()
            .filter(entity_user::Column::Uuid.eq(uuid))
            .one(db)
            .await?
            .ok_or(DbError::NoResult)?
            .into();

        Ok(user)
    }

    pub(crate) async fn get_id_uuid_password_by_email(
        email: String,
        db: &DbConn,
    ) -> DbResult<Option<(i32, Uuid, Option<String>)>> {
        let res = EntityUser::find()
            .filter(entity_user::Column::Email.eq(email))
            .select_only()
            .column(entity_user::Column::Id)
            .column(entity_user::Column::Uuid)
            .column(entity_user::Column::Password)
            .into_tuple()
            .one(db)
            .await?;

        Ok(res)
    }

    pub(crate) async fn update_last_login(
        id: i32,
        db: &DbConn,
    ) -> DbResult<()> {
        let _id = ActiveModelUser {
            id: Unchanged(id),
            last_login: Set(Some(now_utc())),
            ..Default::default()
        }
        .update(db)
        .await?;

        Ok(())
    }
}

impl From<ModelUser> for User {
    fn from(value: ModelUser) -> Self {
        Self {
            id: value.id,
            uuid: value.uuid,
            display_name: value.displayname,
            email: value.email,
            password: value.password,
            last_login: value.last_login,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
