use axum::{
    extract::{rejection::JsonRejection, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    core::{
        api_response::{parse_json, success, ApiError},
        app_state::AppState,
        auth::require_auth,
    },
    domain::models::address::{
        AddressCreateRequest, AddressDeleteResponse, AddressResponse, AddressUpdateRequest,
    },
    infrastructure::repositories::address,
};

fn db_unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "DATABASE_UNAVAILABLE",
        "Database alamat tidak tersedia saat ini.",
    )
}

fn to_response(record: address::AddressRecord) -> AddressResponse {
    AddressResponse {
        id: record.id.to_string(),
        user_id: record.user_id.to_string(),
        label: record.label,
        recipient_name: record.recipient_name,
        recipient_phone: record.recipient_phone,
        full_address: record.full_address,
        additional_details: record.additional_details,
        latitude: record.latitude,
        longitude: record.longitude,
        is_primary: record.is_primary,
    }
}

pub(super) async fn list(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let rows = address::list_by_user(pool, user.id)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(
        rows.into_iter().map(to_response).collect::<Vec<_>>(),
    ))
}

pub(super) async fn create(
    headers: HeaderMap,
    payload: Result<Json<AddressCreateRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = address::create(
        pool,
        address::CreateAddress {
            user_id: user.id,
            label: request.label.as_deref(),
            recipient_name: &request.recipient_name,
            recipient_phone: &request.recipient_phone,
            full_address: &request.full_address,
            additional_details: request.additional_details.as_deref(),
            latitude: request.latitude,
            longitude: request.longitude,
            is_primary: request.is_primary.unwrap_or(false),
        },
    )
    .await
    .map_err(|_| db_unavailable())?;

    Ok((StatusCode::CREATED, success(to_response(record))))
}

pub(super) async fn get(
    headers: HeaderMap,
    Path(address_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = address::find_by_id_and_user(pool, address_id, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::NOT_FOUND,
                "ADDRESS_NOT_FOUND",
                "Alamat tidak ditemukan atau bukan milik Anda.",
            )
        })?;

    Ok(success(to_response(record)))
}

pub(super) async fn update(
    headers: HeaderMap,
    Path(address_id): Path<Uuid>,
    payload: Result<Json<AddressUpdateRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = address::update(
        pool,
        address::UpdateAddress {
            user_id: user.id,
            address_id,
            label: request.label.as_deref(),
            recipient_name: request.recipient_name.as_deref(),
            recipient_phone: request.recipient_phone.as_deref(),
            full_address: request.full_address.as_deref(),
            additional_details: request.additional_details.as_deref(),
            latitude: request.latitude,
            longitude: request.longitude,
            is_primary: request.is_primary,
        },
    )
    .await
    .map_err(|_| db_unavailable())?
    .ok_or_else(|| {
        ApiError::new(
            StatusCode::NOT_FOUND,
            "ADDRESS_NOT_FOUND",
            "Alamat tidak ditemukan atau bukan milik Anda.",
        )
    })?;

    Ok(success(to_response(record)))
}

pub(super) async fn delete(
    headers: HeaderMap,
    Path(address_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let deleted = address::delete_by_id(pool, user.id, address_id)
        .await
        .map_err(|_| db_unavailable())?;

    if !deleted {
        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "ADDRESS_NOT_FOUND",
            "Alamat tidak ditemukan atau bukan milik Anda.",
        ));
    }

    Ok(success(AddressDeleteResponse {
        message: "Alamat berhasil dihapus.",
    }))
}
