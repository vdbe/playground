use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    dto::user::User,
    error::ApiResult,
    service::user::UserService,
    util::validate_payload,
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

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", get(user_get).post(user_post))
}

async fn user_get(State(state): State<AppState>) -> ApiResult<Json<Option<Vec<User>>>> {
    let users = UserService::get_all_users(&state.db)
        .await?;

    Ok(Json(users))
}

async fn user_post(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<Json<User>> {
    validate_payload(&input)?;

    let user = UserService::register_user(input, &state.db).await?;

    Ok(Json(user))
}
