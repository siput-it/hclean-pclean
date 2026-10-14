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
        auth::require_auth,
    },
    domain::models::worker::{WorkerProfileResponse, WorkerProfileUpdateRequest},
    infrastructure::repositories::worker,
};

fn db_unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "DATABASE_UNAVAILABLE",
        "Database pekerja tidak tersedia saat ini.",
    )
}

pub(super) async fn get_me(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat mengakses profil pekerja.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let profile = worker::find_by_user_id(pool, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .unwrap_or_else(|| {
            let mut bio = String::new();
            bio.push_str("");
            worker::WorkerProfileRecord {
                id: uuid::Uuid::new_v4(),
                user_id: user.id,
                bio: Some(bio),
                is_available: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        });

    if profile.user_id == user.id && profile.id != uuid::Uuid::nil() {
        let saved = match worker::find_by_user_id(pool, user.id)
            .await
            .map_err(|_| db_unavailable())?
        {
            Some(existing) => existing,
            None => worker::upsert_profile(pool, user.id, None, true)
                .await
                .map_err(|_| db_unavailable())?,
        };

        return Ok(success(WorkerProfileResponse {
            user_id: saved.user_id.to_string(),
            bio: saved.bio,
            is_available: saved.is_available,
        }));
    }

    Ok(success(WorkerProfileResponse {
        user_id: profile.user_id.to_string(),
        bio: profile.bio,
        is_available: profile.is_available,
    }))
}

pub(super) async fn update_me(
    headers: HeaderMap,
    payload: Result<Json<WorkerProfileUpdateRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat mengakses profil pekerja.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let bio = request
        .bio
        .as_deref()
        .filter(|value| !value.trim().is_empty());
    let is_available = request.is_available.unwrap_or(true);

    let profile = worker::upsert_profile(pool, user.id, bio, is_available)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(WorkerProfileResponse {
        user_id: profile.user_id.to_string(),
        bio: profile.bio,
        is_available: profile.is_available,
    }))
}
