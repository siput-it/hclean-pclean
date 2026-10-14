use axum::Json;
use serde::Serialize;

use crate::core::{
    api_response::{success, ApiSuccess},
    app_state::AppState,
};

#[derive(Serialize)]
pub(super) struct HealthResponse {
    status: &'static str,
    service: &'static str,
    database_configured: bool,
    database_ready: bool,
    database_status: String,
}

pub(super) async fn health() -> Json<ApiSuccess<HealthResponse>> {
    let state = AppState::current();
    let database_status = if state.database_ready() {
        "ready".to_string()
    } else if state.is_database_configured() {
        "not_ready".to_string()
    } else {
        "not_configured".to_string()
    };

    success(HealthResponse {
        status: "ok",
        service: "hclean-pclean-backend",
        database_configured: state.is_database_configured(),
        database_ready: state.database_ready(),
        database_status,
    })
}
