use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{dto::user::UpdateUserInput, util::jwt::ClaimsDecoded};
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
        .route("/me", get(me).patch(update))
}

async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterUserInput>,
) -> ApiResult<StatusCode> {
    validate_payload(&input)?;

    let _uuid = UserService::register_user(input, &state.db).await?;

    Ok(StatusCode::CREATED)
}

#[debug_handler]
async fn update(
    State(state): State<AppState>,
    claims: ClaimsDecoded<SubAccesToken>,
    Json(input): Json<UpdateUserInput>,
) -> ApiResult<()> {
    validate_payload(&input)?;

    let user_uuid = claims.sub().user_uuid;

    UserService::update_by_uuid(user_uuid, input, &state.db).await?;

    Ok(())
}

async fn me(
    State(state): State<AppState>,
    claims: ClaimsDecoded<SubAccesToken>,
) -> ApiResult<Json<User>> {
    let user =
        UserService::get_by_uuid(claims.sub().user_uuid, &state.db).await?;

    Ok(Json(user))
}
