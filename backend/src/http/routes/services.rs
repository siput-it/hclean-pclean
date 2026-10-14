use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

use crate::{
    core::{
        api_response::{success, ApiError, ApiSuccess},
        app_state::AppState,
    },
    domain::models::service::SERVICES,
    infrastructure::repositories::service_category::list_active,
};

#[derive(Serialize, Clone)]
pub(super) struct ServiceSummary {
    id: String,
    name: String,
    description: String,
    price_rupiah: u64,
    price_unit: String,
    simulated: bool,
}

#[derive(Serialize)]
pub(super) struct ServiceList {
    simulated: bool,
    items: Vec<ServiceSummary>,
}

fn fallback_services() -> Vec<ServiceSummary> {
    SERVICES
        .iter()
        .map(|service| ServiceSummary {
            id: service.id.to_string(),
            name: service.name.to_string(),
            description: service.description.to_string(),
            price_rupiah: service.price_rupiah,
            price_unit: service.price_unit.to_string(),
            simulated: service.simulated,
        })
        .collect()
}

pub(super) async fn list() -> Result<Json<ApiSuccess<ServiceList>>, ApiError> {
    let state = AppState::current();
    if let Some(pool) = state.database.as_ref() {
        let rows = list_active(pool).await.map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database layanan tidak tersedia saat ini",
            )
        })?;

        let items = rows
            .into_iter()
            .map(|row| ServiceSummary {
                id: row.id.to_string(),
                name: row.name,
                description: row.description,
                price_rupiah: row.base_price_idr as u64,
                price_unit: "per_order".to_string(),
                simulated: false,
            })
            .collect();

        return Ok(success(ServiceList {
            simulated: false,
            items,
        }));
    }

    Ok(success(ServiceList {
        simulated: true,
        items: fallback_services(),
    }))
}

pub(super) async fn get(
    Path(service_id): Path<String>,
) -> Result<Json<ApiSuccess<ServiceSummary>>, ApiError> {
    let state = AppState::current();
    if let Some(pool) = state.database.as_ref() {
        let rows = list_active(pool).await.map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database layanan tidak tersedia saat ini",
            )
        })?;

        let service = rows.into_iter().find(|row| {
            row.id.to_string() == service_id || row.name.eq_ignore_ascii_case(&service_id)
        });

        if let Some(row) = service {
            return Ok(success(ServiceSummary {
                id: row.id.to_string(),
                name: row.name,
                description: row.description,
                price_rupiah: row.base_price_idr as u64,
                price_unit: "per_order".to_string(),
                simulated: false,
            }));
        }

        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "SERVICE_NOT_FOUND",
            "Layanan tidak ditemukan",
        ));
    }

    let service = fallback_services()
        .into_iter()
        .find(|item| item.id == service_id || item.name.eq_ignore_ascii_case(&service_id));

    match service {
        Some(item) => Ok(success(item)),
        None => Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "SERVICE_NOT_FOUND",
            "Layanan tidak ditemukan",
        )),
    }
}
