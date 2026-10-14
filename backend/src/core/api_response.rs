use axum::{
    extract::{rejection::JsonRejection, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Serialize)]
pub struct ApiSuccess<T> {
    success: bool,
    data: T,
}

pub fn success<T: Serialize>(data: T) -> Json<ApiSuccess<T>> {
    Json(ApiSuccess {
        success: true,
        data,
    })
}

#[derive(Serialize)]
struct ApiErrorBody {
    success: bool,
    error: ApiErrorDetail,
}

#[derive(Serialize)]
struct ApiErrorDetail {
    code: &'static str,
    message: &'static str,
}

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl ApiError {
    pub const fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            code,
            message,
        }
    }

    pub fn validation() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            "Data permintaan tidak valid",
        )
    }

    pub fn not_implemented(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED, code, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ApiErrorBody {
                success: false,
                error: ApiErrorDetail {
                    code: self.code,
                    message: self.message,
                },
            }),
        )
            .into_response()
    }
}

pub fn parse_json<T: DeserializeOwned>(
    payload: Result<Json<T>, JsonRejection>,
) -> Result<T, ApiError> {
    payload
        .map(|Json(value)| value)
        .map_err(|_| ApiError::validation())
}
