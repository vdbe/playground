use std::time::Duration;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::constant::{BEARER, REFRESH_TOKEN_TIMEOUT},
    dto::{
        auth::{LoginPayload, RefreshPayload, SubAccesToken, SubRefreshToken},
        user::LoginUserInput,
    },
    error::ApiResult,
    service::user::UserService,
    util::{
        jwt::{self, Claims, ClaimsDecoded},
        validate_payload,
    },
    AppState,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
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

    let sub_refresh_token = SubRefreshToken::new(refresh_token.token);
    let sub_access_token = SubAccesToken::new(uuid);

    let claim_refresh_token = Claims::new(sub_refresh_token)?;
    let claim_access_token = Claims::new(sub_access_token)?;

    let login_payload = LoginPayload {
        refresh_token: claim_refresh_token,
        access_token: RefreshPayload {
            access_token: claim_access_token,
            token_type: BEARER.to_string(),
        },
    };

    Ok(Json(login_payload))
}

async fn logout(
    State(state): State<AppState>,
    claims: ClaimsDecoded<SubRefreshToken>,
) -> ApiResult<()> {
    UserService::logout(claims.sub().token, &state.db).await?;

    Ok(())
}

async fn refresh(
    State(state): State<AppState>,
    claims: ClaimsDecoded<SubRefreshToken>,
) -> ApiResult<Json<RefreshPayload>> {
    let (_refresh_token, user) =
        UserService::verify_refresh_token(claims.sub().token, &state.db)
            .await?;

    let claim_access_token = Claims::new(SubAccesToken::new(user.uuid))?;

    let refresh_payload = RefreshPayload {
        access_token: claim_access_token,
        token_type: BEARER.to_string(),
    };

    Ok(Json(refresh_payload))
}

async fn me(
    State(_state): State<AppState>,
    claims: ClaimsDecoded<SubAccesToken>,
) -> ApiResult<Json<jwt::Decoded<SubAccesToken>>> {
    Ok(Json(claims.claims()))
}
