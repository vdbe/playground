use std::time::Duration;

use uuid::Uuid;

use crate::{
    dto::{
        auth::RefreshToken,
        user::{LoginUserInput, RegisterUserInput, UpdateUserInput, User},
    },
    error::{ErrorRepr, ResultRepr, UserError},
    util::{
        encryption::{hash_password, verify_password},
        now_utc,
    },
    DbConn,
};

pub(crate) struct UserService;

impl UserService {
    pub(crate) async fn create_refresh_token(
        user_uuid: Uuid,
        duration: Duration,
        db: &DbConn,
    ) -> ResultRepr<RefreshToken> {
        // FIX: Mismatch between expiry_date in db and claim
        let expiry_date = now_utc() + duration;

        let refresh_token =
            RefreshToken::new(user_uuid, expiry_date, db).await?;

        Ok(refresh_token)
    }

    pub(crate) async fn verify_refresh_token(
        token: Uuid,
        db: &DbConn,
    ) -> ResultRepr<(RefreshToken, User)> {
        let res = RefreshToken::get_user_by_token(token, db).await?;

        Ok(res)
    }

    pub(crate) async fn get_by_uuid(
        uuid: Uuid,
        db: &DbConn,
    ) -> ResultRepr<User> {
        let res = User::get_by_uuid(uuid, db).await?;

        Ok(res)
    }

    pub(crate) async fn update_by_uuid(
        uuid: Uuid,
        mut update_user_input: UpdateUserInput,
        db: &DbConn,
    ) -> ResultRepr<()> {
        // Checks if all fields are `None`
        if update_user_input == Default::default() {
            return Ok(());
        }

        // Hash password if not `None`/`Some(None)`
        let password = if let Some(password) = update_user_input.password {
            let password = if let Some(password) = password {
                // TODO: Check if it matches previous password?
                Some(hash_password(password).await?)
            } else {
                None
            };

            Some(password)
        } else {
            None
        };

        update_user_input.password = password;

        User::update_by_uuid(uuid, update_user_input, db).await?;

        Ok(())
    }

    pub(crate) async fn login(
        input: LoginUserInput,
        db: &DbConn,
    ) -> ResultRepr<Uuid> {
        let password = input.password.ok_or(UserError::PasswordRequired)?;

        let result =
            User::get_id_uuid_password_by_email(input.email, db).await?;

        let (id, uuid, password_hash) = result.ok_or(UserError::NotFound)?;

        let password_hash = password_hash.ok_or(UserError::NoPassword)?;

        if verify_password(password, password_hash).await? {
            User::update_last_login(id, db).await?;

            Ok(uuid)
        } else {
            Err(ErrorRepr::User(UserError::PasswordWrong))
        }
    }

    pub(crate) async fn logout(token: Uuid, db: &DbConn) -> ResultRepr<()> {
        RefreshToken::drop_by_token(token, db).await?;

        Ok(())
    }

    pub(crate) async fn register_user(
        input: RegisterUserInput,
        db: &DbConn,
    ) -> ResultRepr<User> {
        // TODO: Check if displayname/email already exists

        let now = now_utc();

        let password = if let Some(password) = input.password {
            Some(hash_password(password).await?)
        } else {
            None
        };

        let user = User {
            display_name: input.display_name,
            email: input.email,
            password,
            uuid: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            ..Default::default()
        };
        let user = user.create(db).await?;

        Ok(user)
    }
}
