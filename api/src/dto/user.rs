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
    #[serde(skip)]
    pub(crate) id: i32,
    #[serde(skip)]
    pub(crate) uuid: Uuid,
    pub(crate) display_name: String,
    pub(crate) email: String,
    #[serde(skip)]
    pub(crate) password: Option<String>,
    #[serde(skip)]
    pub(crate) last_login: Option<PrimitiveDateTime>,
    #[serde(skip)]
    pub(crate) created_at: PrimitiveDateTime,
    #[serde(skip)]
    pub(crate) updated_at: PrimitiveDateTime,
}

impl Default for User {
    fn default() -> Self {
        let now = now_utc();

        Self {
            id: Default::default(),
            uuid: Uuid::new_v4(),
            display_name: Default::default(),
            email: Default::default(),
            password: Default::default(),
            last_login: None,
            created_at: now,
            updated_at: now,
        }
    }
}
