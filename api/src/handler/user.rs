use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::util::jwt::ClaimsDecoded;
use crate::{
    dto::{
        auth::SubAccesToken,
        user::{RegisterUserInput, User},
    },
    error::ApiResult,
    service::user::UserService,
    util::validate_payload,
    AppState,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(register))
        .route("/me", get(me))
}

async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<StatusCode> {
    validate_payload(&input)?;

    let _uuid = UserService::register_user(input, &state.db).await?;

    Ok(StatusCode::CREATED)
}

async fn me(
    State(state): State<AppState>,
    claims: ClaimsDecoded<SubAccesToken>,
) -> ApiResult<Json<User>> {
    let user = UserService::get_by_uuid(claims.sub().uuid, &state.db).await?;

    Ok(Json(user))
}
