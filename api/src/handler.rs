use axum::{extract::State, routing::get, Json, Router};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;
use uuid::Uuid;

use entity::user::{self as user_table, Entity as EntityUser, Model as ModelUser};

use crate::{
    error::{ApiResult, Result},
    AppState,
};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub(crate) struct RegisterUserInput {
    #[validate(length(min = 2, max = 20))]
    pub(crate) display_name: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 6))]
    pub(crate) password: Option<String>,
}

pub(crate) fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(user_get).post(user_post))
}

async fn user_get(State(state): State<AppState>) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let users: Vec<serde_json::Value> = EntityUser::find().into_json().all(&state.db).await.unwrap();

    Ok(Json(users))
}

async fn user_post(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<Json<ModelUser>> {
    validate_payload(&input)?;

    let user = register(input, &state.db).await?;

    Ok(Json(user))
}

async fn register(register_input: RegisterUserInput, db: &DatabaseConnection) -> Result<ModelUser> {
    let user = user_table::ActiveModel {
        displayname: Set(register_input.display_name),
        email: Set(register_input.email),
        password: Set(register_input.password),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    };

    Ok(user.insert(db).await?)
}
