use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{
    dto::user::RegisterUserInput, error::ApiResult, service::user::UserService,
    util::validate_payload, AppState,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/", post(user_register))
}

async fn user_register(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<StatusCode> {
    validate_payload(&input)?;

    let _uuid = UserService::register_user(input, &state.db).await?;

    Ok(StatusCode::CREATED)
}
