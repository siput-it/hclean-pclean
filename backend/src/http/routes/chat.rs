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
    domain::models::chat::{CreateMessageRequest, Message},
    infrastructure::repositories::{
        conversation as conversation_repo, message as message_repo,
        notification as notification_repo, order as order_repo,
    },
};

fn db_unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "DATABASE_UNAVAILABLE",
        "Database chat tidak tersedia saat ini.",
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

fn order_not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "ORDER_NOT_FOUND",
        "Pesanan tidak ditemukan atau tidak dapat diakses.",
    )
}

fn has_order_access(user_role: &str, user_id: Uuid, order: &order_repo::OrderRecord) -> bool {
    if user_role == "customer" {
        return order.customer_id == user_id;
    }

    if user_role == "worker" {
        return order.worker_id == Some(user_id);
    }

    false
}

pub(super) async fn list_messages(
    headers: HeaderMap,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let order = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if !has_order_access(&user.role, user.id, &order) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "CHAT_ACCESS_FORBIDDEN",
            "Anda tidak memiliki akses ke percakapan pesanan ini.",
        ));
    }

    let _conversation = conversation_repo::ensure_for_order(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?;
    let records = message_repo::list_for_order(pool, order_id, 50, 0)
        .await
        .map_err(|_| db_unavailable())?;

    let messages = records
        .into_iter()
        .map(|record| Message {
            id: record.id.to_string(),
            conversation_id: record.conversation_id.to_string(),
            sender_id: record.sender_id.to_string(),
            body: record.body,
            created_at: record.created_at.to_rfc3339(),
        })
        .collect::<Vec<_>>();

    Ok(success(messages))
}

pub(super) async fn create_message(
    headers: HeaderMap,
    Path(order_id): Path<String>,
    payload: Result<Json<CreateMessageRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let order_id = parse_order_id(&order_id)?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(db_unavailable)?;

    let order = order_repo::find_by_id(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?
        .ok_or_else(order_not_found)?;

    if !has_order_access(&user.role, user.id, &order) {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "CHAT_ACCESS_FORBIDDEN",
            "Anda tidak memiliki akses untuk mengirim pesan pada pesanan ini.",
        ));
    }

    let conversation = conversation_repo::ensure_for_order(pool, order_id)
        .await
        .map_err(|_| db_unavailable())?;
    let record = message_repo::create(pool, conversation.id, user.id, request.body.trim())
        .await
        .map_err(|_| db_unavailable())?;

    let recipient = if order.customer_id == user.id {
        order.worker_id
    } else {
        Some(order.customer_id)
    };

    if let Some(recipient_id) = recipient {
        let _ = notification_repo::create_for_user(
            pool,
            recipient_id,
            "new_message",
            "Pesan baru",
            "Anda menerima pesan baru untuk pesanan ini.",
            Some("order"),
            Some(order.id),
        )
        .await;
    }

    Ok((
        StatusCode::CREATED,
        success(Message {
            id: record.id.to_string(),
            conversation_id: record.conversation_id.to_string(),
            sender_id: record.sender_id.to_string(),
            body: record.body,
            created_at: record.created_at.to_rfc3339(),
        }),
    ))
}
