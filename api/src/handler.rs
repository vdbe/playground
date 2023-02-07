use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use crate::{
    dto::user::{LoginUserInput, RegisterUserInput, User},
    error::ApiResult,
    service::user::UserService,
    util::{jwt, validate_payload},
    AppState,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_get).post(user_register))
        .route("/login", post(user_login))
}

async fn user_get(State(state): State<AppState>) -> ApiResult<Json<Option<Vec<User>>>> {
    let users = UserService::get_all_users(&state.db).await?;

    Ok(Json(users))
}

async fn user_register(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<Json<User>> {
    validate_payload(&input)?;

    let user = UserService::register_user(input, &state.db).await?;

    Ok(Json(user))
}

async fn user_login(
    State(state): State<AppState>,
    Json(input): Json<LoginUserInput>,
) -> ApiResult<Json<User>> {
    validate_payload(&input)?;

    let user = UserService::login_user(input, &state.db).await?;

    Ok(Json(user))
}
