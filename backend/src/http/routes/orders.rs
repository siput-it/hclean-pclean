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
    domain::models::order::{CreateOrderRequest, Order, OrderStatus},
    infrastructure::repositories::{
        address, notification as notification_repo, order as order_repo, service_category,
    },
};

fn db_unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "DATABASE_UNAVAILABLE",
        "Database pesanan tidak tersedia saat ini.",
    )
}

fn order_to_response(record: &order_repo::OrderRecord) -> Order {
    Order {
        id: record.id.to_string(),
        service_id: record.service_category_id.to_string(),
        status: OrderStatus::parse(record.status.as_str()).unwrap_or(OrderStatus::Pending),
        scheduled_at: record.created_at.to_rfc3339(),
        address: record.address.clone().unwrap_or_default(),
        price_rupiah: record.service_price_idr as u64,
        customer_service_fee_rupiah: record.customer_fee_idr as u64,
        worker_service_fee_rupiah: record.worker_fee_idr as u64,
        simulated: false,
    }
}

fn order_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "ORDER_NOT_FOUND",
        "Pesanan tidak ditemukan atau tidak dapat diakses.",
    )
}

fn parse_order_id(value: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(value.trim()).map_err(|_| {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            "Format ID pesanan tidak valid.",
        )
    })
}

pub(super) async fn create(
    headers: HeaderMap,
    payload: Result<Json<CreateOrderRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    if user.role != "customer" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "CUSTOMER_REQUIRED",
            "Hanya pelanggan yang dapat membuat pesanan.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let service_id_value = request
        .service_category_id
        .as_deref()
        .or(request.service_id.as_deref())
        .ok_or_else(ApiError::validation)?;
    let service_category_id = parse_order_id(service_id_value)?;

    let address_id_value = request
        .address_id
        .as_deref()
        .ok_or_else(ApiError::validation)?;
    let address_id = parse_order_id(address_id_value)?;

    let service = service_category::find_by_id(pool, service_category_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::NOT_FOUND,
                "SERVICE_NOT_FOUND",
                "Kategori layanan tidak ditemukan atau tidak aktif.",
            )
        })?;

    let address = address::find_by_id_and_user(pool, address_id, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::NOT_FOUND,
                "ADDRESS_NOT_FOUND",
                "Alamat tidak ditemukan atau bukan milik Anda.",
            )
        })?;

    let service_price_idr = service.base_price_idr as i64;
    let customer_fee_idr = (service_price_idr * 20) / 100;
    let worker_fee_idr = customer_fee_idr;

    let record = order_repo::create_order(
        pool,
        order_repo::CreateOrder {
            customer_id: user.id,
            service_category_id: service.id,
            address_id: address.id,
            address_snapshot: &address.full_address,
            notes: request.notes.as_deref(),
            service_price_idr,
            customer_fee_idr,
            worker_fee_idr,
        },
    )
    .await
    .map_err(|_| db_unavailable())?;

    Ok((StatusCode::CREATED, success(order_to_response(&record))))
}

pub(super) async fn list(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "customer" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "CUSTOMER_REQUIRED",
            "Hanya pelanggan yang dapat melihat pesanan pelanggan.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let orders = order_repo::list_for_customer(pool, user.id, 50, 0)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(
        orders.iter().map(order_to_response).collect::<Vec<_>>(),
    ))
}

pub(super) async fn get(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if user.role == "customer" && record.customer_id == user.id {
        return Ok(success(order_to_response(&record)));
    }

    if user.role == "worker" && (record.worker_id == Some(user.id) || record.status == "pending") {
        return Ok(success(order_to_response(&record)));
    }

    Err(order_not_found())
}

pub(super) async fn worker_available(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat melihat pesanan tersedia.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let orders = order_repo::list_available_for_worker(pool, 50, 0)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(
        orders.iter().map(order_to_response).collect::<Vec<_>>(),
    ))
}

pub(super) async fn worker_list(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat melihat tugas pekerja.",
        ));
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let orders = order_repo::list_for_worker(pool, user.id, 50, 0)
        .await
        .map_err(|_| db_unavailable())?;

    Ok(success(
        orders.iter().map(order_to_response).collect::<Vec<_>>(),
    ))
}

pub(super) async fn worker_get(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat membuka detail pesanan pekerja.",
        ));
    }

    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let record = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if record.worker_id == Some(user.id) || record.status == "pending" {
        return Ok(success(order_to_response(&record)));
    }

    Err(order_not_found())
}

pub(super) async fn accept(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat menerima pesanan.",
        ));
    }

    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let current = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if current.status != "pending" || current.worker_id.is_some() {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "ORDER_ALREADY_ACCEPTED",
            "Pesanan tidak dapat diterima karena statusnya sudah berubah.",
        ));
    }

    let record = order_repo::accept_order(pool, order_id, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    let _ = notification_repo::create_for_user(
        pool,
        current.customer_id,
        "order_accepted",
        "Pesanan diterima",
        "Pekerja Anda sudah menerima pesanan Anda.",
        Some("order"),
        Some(order_id),
    )
    .await;

    Ok(success(order_to_response(&record)))
}

pub(super) async fn start(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat memulai pekerjaan.",
        ));
    }

    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let current = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if current.worker_id != Some(user.id) || current.status != "accepted" {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "ORDER_STATUS_CONFLICT",
            "Pesanan hanya dapat dimulai oleh pekerja yang menerima tugas dan masih dalam status accepted.",
        ));
    }

    let record = order_repo::start_order(pool, order_id, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    let _ = notification_repo::create_for_user(
        pool,
        current.customer_id,
        "order_started",
        "Pekerjaan dimulai",
        "Pekerja telah mulai mengerjakan pesanan Anda.",
        Some("order"),
        Some(order_id),
    )
    .await;

    Ok(success(order_to_response(&record)))
}

pub(super) async fn complete(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    if user.role != "worker" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "WORKER_REQUIRED",
            "Hanya pekerja yang dapat menandai pekerjaan selesai.",
        ));
    }

    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let current = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if current.worker_id != Some(user.id) || current.status != "in_progress" {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "ORDER_STATUS_CONFLICT",
            "Pesanan hanya dapat diselesaikan saat sedang dikerjakan oleh pekerja yang sama.",
        ));
    }

    let record = order_repo::complete_order(pool, order_id, user.id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    let _ = notification_repo::create_for_user(
        pool,
        current.customer_id,
        "order_completed",
        "Pekerjaan selesai",
        "Pekerjaan untuk pesanan Anda telah selesai.",
        Some("order"),
        Some(order_id),
    )
    .await;

    Ok(success(order_to_response(&record)))
}

pub(super) async fn cancel(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let current = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    let allowed = match user.role.as_str() {
        "customer" => {
            current.customer_id == user.id
                && matches!(current.status.as_str(), "pending" | "accepted")
        }
        "worker" => {
            current.worker_id == Some(user.id)
                && matches!(current.status.as_str(), "accepted" | "in_progress")
        }
        _ => false,
    };

    if !allowed {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "ORDER_CANNOT_BE_CANCELLED",
            "Status pesanan saat ini tidak dapat dibatalkan oleh pengguna yang terautentikasi.",
        ));
    }

    let record = order_repo::cancel_order(pool, order_id, user.id, &user.role, None)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    let target_user = if user.role == "customer" {
        current.worker_id
    } else {
        Some(current.customer_id)
    };

    if let Some(target_user_id) = target_user {
        let _ = notification_repo::create_for_user(
            pool,
            target_user_id,
            "order_cancelled",
            "Pesanan dibatalkan",
            "Pesanan terkait telah dibatalkan.",
            Some("order"),
            Some(order_id),
        )
        .await;
    }

    Ok(success(order_to_response(&record)))
}
