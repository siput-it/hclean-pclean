use axum::Json;
use serde::Serialize;

use crate::core::api_response::{success, ApiSuccess};

#[derive(Serialize)]
pub(super) struct ApiInfo {
    version: &'static str,
    service: &'static str,
    message: &'static str,
}

pub(super) async fn info() -> Json<ApiSuccess<ApiInfo>> {
    success(ApiInfo {
        version: "v1",
        service: "hclean-pclean-backend",
        message: "API ready",
    })
}
