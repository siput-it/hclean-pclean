use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    core::{
        api_response::{success, ApiError},
        app_state::AppState,
        auth::require_auth,
    },
    domain::models::notification::Notification,
    infrastructure::repositories::notification as notification_repo,
};

fn db_unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "DATABASE_UNAVAILABLE",
        "Database notifikasi tidak tersedia saat ini.",
    )
}

fn parse_notification_id(value: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(value.trim()).map_err(|_| {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            "Format ID notifikasi tidak valid.",
        )
    })
}

pub(super) async fn list(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let records = notification_repo::list_for_user(pool, user.id, 20, 0)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(
        records
            .into_iter()
            .map(|record| Notification {
                id: record.id.to_string(),
                user_id: record.user_id.to_string(),
                r#type: record.type_.to_string(),
                title: record.title,
                body: record.body,
                resource_type: record.resource_type,
                resource_id: record.resource_id.map(|id| id.to_string()),
                read_at: record.read_at.map(|value| value.to_rfc3339()),
                created_at: record.created_at.to_rfc3339(),
            })
            .collect::<Vec<_>>(),
    ))
}

pub(super) async fn unread_count(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let total = notification_repo::unread_count(pool, user.id)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(serde_json::json!({ "unread_count": total })))
}

pub(super) async fn mark_read(
    headers: HeaderMap,
    Path(notification_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let notification_id = parse_notification_id(&notification_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = notification_repo::mark_read(pool, user.id, notification_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::NOT_FOUND,
                "NOTIFICATION_NOT_FOUND",
                "Notifikasi tidak ditemukan atau bukan milik Anda.",
            )
        })?;

    Ok(success(Notification {
        id: record.id.to_string(),
        user_id: record.user_id.to_string(),
        r#type: record.type_.to_string(),
        title: record.title,
        body: record.body,
        resource_type: record.resource_type,
        resource_id: record.resource_id.map(|id| id.to_string()),
        read_at: record.read_at.map(|value| value.to_rfc3339()),
        created_at: record.created_at.to_rfc3339(),
    }))
}

pub(super) async fn mark_all_read(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let updated = notification_repo::mark_all_read(pool, user.id)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(serde_json::json!({ "updated_count": updated })))
}
