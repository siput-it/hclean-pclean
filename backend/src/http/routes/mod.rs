mod addresses;
mod api_info;
mod auth;
mod chat;
mod health;
mod notifications;
mod orders;
mod services;
mod users;
mod wallet;
mod workers;

use axum::{routing::get, Router};

use crate::core::api_response::ApiError;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/api/v1", get(api_info::info))
        .route("/api/v1/auth/register", axum::routing::post(auth::register))
        .route("/api/v1/auth/login", axum::routing::post(auth::login))
        .route("/api/v1/auth/logout", axum::routing::post(auth::logout))
        .route(
            "/api/v1/users/me",
            get(users::get_me).patch(users::update_me),
        )
        .route(
            "/api/v1/workers/me",
            get(workers::get_me).patch(workers::update_me),
        )
        .route(
            "/api/v1/addresses",
            get(addresses::list).post(addresses::create),
        )
        .route(
            "/api/v1/addresses/{address_id}",
            get(addresses::get)
                .patch(addresses::update)
                .delete(axum::routing::delete(addresses::delete)),
        )
        .route("/api/v1/services", get(services::list))
        .route("/api/v1/services/{service_id}", get(services::get))
        .route("/api/v1/orders", get(orders::list).post(orders::create))
        .route("/api/v1/orders/{order_id}", get(orders::get))
        .route(
            "/api/v1/worker/orders/available",
            get(orders::worker_available),
        )
        .route("/api/v1/worker/orders", get(orders::worker_list))
        .route("/api/v1/worker/orders/{order_id}", get(orders::worker_get))
        .route(
            "/api/v1/orders/{order_id}/accept",
            axum::routing::post(orders::accept),
        )
        .route(
            "/api/v1/orders/{order_id}/start",
            axum::routing::post(orders::start),
        )
        .route(
            "/api/v1/orders/{order_id}/complete",
            axum::routing::post(orders::complete),
        )
        .route(
            "/api/v1/orders/{order_id}/cancel",
            axum::routing::post(orders::cancel),
        )
        .route(
            "/api/v1/orders/{order_id}/messages",
            get(chat::list_messages).post(chat::create_message),
        )
        .route("/api/v1/notifications", get(notifications::list))
        .route(
            "/api/v1/notifications/unread-count",
            get(notifications::unread_count),
        )
        .route(
            "/api/v1/notifications/{notification_id}/read",
            axum::routing::patch(notifications::mark_read),
        )
        .route(
            "/api/v1/notifications/read-all",
            axum::routing::post(notifications::mark_all_read),
        )
        .route("/api/v1/wallet", get(wallet::summary))
        .route("/api/v1/wallet/transactions", get(wallet::transactions))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
}

async fn not_found() -> ApiError {
    ApiError::new(
        axum::http::StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "Endpoint tidak ditemukan",
    )
}

async fn method_not_allowed() -> ApiError {
    ApiError::new(
        axum::http::StatusCode::METHOD_NOT_ALLOWED,
        "METHOD_NOT_ALLOWED",
        "Metode HTTP tidak diizinkan untuk endpoint ini",
    )
}
