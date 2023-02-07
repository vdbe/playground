use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use entity::user::{self as user_table, Entity as EntityUser, Model as ModelUser};
use uuid::Uuid;

use crate::{
    dto::user::{LoginUserInput, RegisterUserInput, User},
    error::{ErrorRepr, ResultRepr, UserError},
    util::{
        encryption::{hash_password, verify_password},
        into_option_vec, now_utc,
    },
    DbConn,
};

pub(crate) struct UserService;

impl UserService {
    pub(crate) async fn get_all_users(db: &DbConn) -> ResultRepr<Option<Vec<User>>> {
        let users: Vec<ModelUser> = EntityUser::find().all(db).await?;

        Ok(into_option_vec(users))
    }

    pub(crate) async fn login_user(input: LoginUserInput, db: &DbConn) -> ResultRepr<User> {
        let password = input.password.ok_or(UserError::PasswordRequired)?;

        let model_user = EntityUser::find()
            .filter(user_table::Column::Email.eq(input.email))
            .one(db)
            .await?
            .ok_or(UserError::NotFound)?;

        let password_hash = model_user
            .password
            .as_ref()
            .ok_or(UserError::NoPassword)?
            .to_string();

        let model_user = Self::update_last_login(model_user.into(), db).await?;

        if verify_password(password, password_hash).await? {
            Ok(model_user.into())
        } else {
            Err(ErrorRepr::User(UserError::PasswordWrong))
        }
    }
    pub(crate) async fn register_user(input: RegisterUserInput, db: &DbConn) -> ResultRepr<User> {
        // TODO: Check if displayname/email already exists

        let now = now_utc();

        let password = if let Some(password) = input.password {
            Some(hash_password(password).await?)
        } else {
            None
        };

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

    async fn update_last_login(
        mut user: user_table::ActiveModel,
        db: &DbConn,
    ) -> ResultRepr<ModelUser> {
        user.last_login = Set(Some(now_utc()));

        Ok(user.update(db).await?)
    }
}
