use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use entity::user::{self as user_table, Entity as EntityUser, Model as ModelUser};
use uuid::Uuid;

use crate::{
    dto::user::User,
    error::ResultRepr,
    handler::RegisterUserInput,
    util::{encryption::hash_password, into_option_vec, now_utc},
    DbConn,
};

pub(crate) struct UserService;

impl UserService {
    pub(crate) async fn get_all_users(db: &DbConn) -> ResultRepr<Option<Vec<User>>> {
        let users: Vec<ModelUser> = EntityUser::find().all(db).await?;

        Ok(into_option_vec(users))
    }

    pub(crate) async fn register_user(input: RegisterUserInput, db: &DbConn) -> ResultRepr<User> {
        // TODO: Check if displayname/email already exists

        let now = now_utc();

        let password = if let Some(password) = input.password {
            Some(hash_password(password).await?)
        } else { None };

        let user = user_table::ActiveModel {
            displayname: Set(input.display_name),
            email: Set(input.email),
            password: Set(password),
            uuid: Set(Uuid::new_v4()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let user: ModelUser = user.insert(db).await?;

        Ok(user.into())
    }
}
