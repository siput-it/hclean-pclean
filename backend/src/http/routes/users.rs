use axum::{
    extract::rejection::JsonRejection,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::{
    core::{
        api_response::{parse_json, success, ApiError},
        app_state::AppState,
        auth::{normalize_phone, require_auth, PublicUser},
    },
    domain::models::auth::UpdateProfileRequest,
    infrastructure::repositories::user::{find_by_id, update_profile},
};

pub(super) async fn get_me(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let user_record = find_by_id(pool, user.id)
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database autentikasi tidak tersedia saat ini.",
            )
        })?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "INVALID_TOKEN",
                "Token tidak valid atau kedaluwarsa.",
            )
        })?;

    Ok(success(PublicUser::from(&user_record)))
}

pub(super) async fn update_me(
    headers: HeaderMap,
    payload: Result<Json<UpdateProfileRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    if let Some(phone_number) = request.phone_number.as_deref().and_then(normalize_phone) {
        let duplicate = crate::infrastructure::repositories::user::find_by_email_or_phone(
            pool,
            None,
            Some(&phone_number),
        )
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database autentikasi tidak tersedia saat ini.",
            )
        })?;

        if duplicate.is_some_and(|record| record.id != user.id) {
            return Err(ApiError::new(
                StatusCode::CONFLICT,
                "ACCOUNT_ALREADY_EXISTS",
                "Nomor telepon sudah digunakan oleh akun lain.",
            ));
        }
    }

    let updated_user = update_profile(
        pool,
        user.id,
        request.name.as_deref(),
        request.phone_number.as_deref(),
    )
    .await
    .map_err(|_| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    Ok(success(PublicUser::from(&updated_user)))
}
