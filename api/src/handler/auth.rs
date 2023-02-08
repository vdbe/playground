use std::time::Duration;

use axum::{extract::State, routing::post, Json, Router};

use crate::{
    config::constant::{BEARER, REFRESH_TOKEN_TIMEOUT},
    dto::{
        auth::{LoginPayload, RefreshPayload, SubAccesToken, SubRefreshToken},
        user::LoginUserInput,
    },
    error::ApiResult,
    service::user::UserService,
    util::{
        jwt::{ClaimSub, Claims},
        validate_payload,
    },
    AppState,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", post(me))
}

async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginUserInput>,
) -> ApiResult<Json<LoginPayload>> {
    validate_payload(&input)?;

    let uuid = UserService::login(input, &state.db).await?;
    let refresh_token = UserService::create_refresh_token(
        uuid,
        Duration::from_secs(REFRESH_TOKEN_TIMEOUT),
        &state.db,
    )
    .await?;

    let claim_refresh_token = SubRefreshToken::new(refresh_token.token).claim()?;
    let claim_access_token = SubAccesToken::new(uuid).claim()?;

    let login_payload = LoginPayload {
        refresh_token: claim_refresh_token,
        access_token: RefreshPayload {
            access_token: claim_access_token,
            token_type: BEARER.to_string(),
        },
    };

    Ok(Json(login_payload))
}

async fn logout(State(state): State<AppState>, claims: Claims<SubRefreshToken>) -> ApiResult<()> {
    UserService::logout(claims.sub.token, &state.db).await?;

    Ok(())
}
async fn refresh(
    State(state): State<AppState>,
    claims: Claims<SubRefreshToken>,
) -> ApiResult<Json<RefreshPayload>> {
    let (_refresh_token, user) =
        UserService::verify_refresh_token(claims.sub.token, &state.db).await?;

    let claim_access_token = SubAccesToken::new(user.uuid).claim()?;

    let refresh_payload = RefreshPayload {
        access_token: claim_access_token,
        token_type: BEARER.to_string(),
    };

    Ok(Json(refresh_payload))
}
async fn me(
    State(_state): State<AppState>,
    claims: Claims<SubAccesToken>,
) -> ApiResult<Json<Claims<SubAccesToken>>> {
    Ok(Json(claims))
}
