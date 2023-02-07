use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::util::now_utc;

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct LoginUserInput {
    // NOTE: Maybe don't validate email
    // The validation email filter may change or there could be a manual
    // approved login in the db that is not an email
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct RegisterUserInput {
    #[validate(length(min = 2, max = 20))]
    pub(crate) display_name: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 6))]
    pub(crate) password: Option<String>,
}

#[derive(Debug, Serialize, Validate)]
pub(crate) struct User {
    //#[serde(skip)]
    //id: i32,
    #[serde(skip)]
    uuid: Uuid,
    display_name: String,
    email: String,
    //#[serde(skip)]
    //password: Option<String>,
    #[serde(skip)]
    last_login: Option<PrimitiveDateTime>,
    #[serde(skip)]
    created_at: PrimitiveDateTime,
    #[serde(skip)]
    updated_at: PrimitiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        let now = now_utc();

        Self {
            //id: Default::default(),
            uuid: Uuid::new_v4(),
            display_name: Default::default(),
            email: Default::default(),
            //password: Default::default(),
            last_login: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<entity::user::Model> for User {
    fn from(value: entity::user::Model) -> Self {
        Self {
            //id: value.id,
            uuid: value.uuid,
            display_name: value.displayname,
            email: value.email,
            //password: value.password,
            last_login: value.last_login,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
