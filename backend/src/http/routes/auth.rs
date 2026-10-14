use axum::{
    extract::rejection::JsonRejection,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    core::{
        api_response::{parse_json, success, ApiError},
        app_state::AppState,
        auth::{
            generate_tokens, hash_password, normalize_email, normalize_phone, require_auth,
            verify_password, PublicUser,
        },
    },
    domain::models::auth::{LoginRequest, RegisterRequest, UserRole},
    infrastructure::repositories::{session, user},
};

#[derive(Serialize)]
struct RegisterResponse {
    user: PublicUser,
}

#[derive(Serialize)]
struct LoginResponse {
    user: PublicUser,
    access_token: String,
    refresh_token: String,
    token_type: &'static str,
    expires_in_seconds: usize,
    refresh_expires_in_seconds: usize,
}

#[derive(Serialize)]
struct LogoutResponse {
    message: &'static str,
}

fn role_as_str(role: UserRole) -> &'static str {
    match role {
        UserRole::Customer => "customer",
        UserRole::Worker => "worker",
    }
}

fn hash_token(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(super) async fn register(
    payload: Result<Json<RegisterRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let email = request.email.as_deref().and_then(normalize_email);
    let phone = request.phone_number.as_deref().and_then(normalize_phone);

    if email.is_none() && phone.is_none() {
        return Err(ApiError::validation());
    }

    let existing = user::find_by_email_or_phone(pool, email.as_deref(), phone.as_deref())
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database autentikasi tidak tersedia saat ini.",
            )
        })?;

    if existing.is_some() {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "ACCOUNT_ALREADY_EXISTS",
            "Email atau nomor telepon sudah terdaftar.",
        ));
    }

    let password_hash = hash_password(&request.password)?;
    let created_user = user::create_user(
        pool,
        email.as_deref(),
        phone.as_deref(),
        &password_hash,
        role_as_str(request.role),
        request.name.trim(),
    )
    .await
    .map_err(|_| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let response = RegisterResponse {
        user: PublicUser::from(&created_user),
    };

    Ok((StatusCode::CREATED, success(response)))
}

pub(super) async fn login(
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let request = parse_json(payload)?;
    if !request.is_valid() {
        return Err(ApiError::validation());
    }

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let email = request.email.as_deref().and_then(normalize_email);
    let phone = request.phone_number.as_deref().and_then(normalize_phone);

    let user_record = user::find_by_email_or_phone(pool, email.as_deref(), phone.as_deref())
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database autentikasi tidak tersedia saat ini.",
            )
        })?
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Kredensial tidak valid.",
            )
        })?;

    let password_hash = user_record.password_hash.as_deref().ok_or_else(|| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "Kredensial tidak valid.",
        )
    })?;

    if !verify_password(&request.password, password_hash)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "Kredensial tidak valid.",
        ));
    }

    let tokens = generate_tokens(&user_record)?;
    let access_hash = hash_token(&tokens.access_token);
    let refresh_hash = hash_token(&tokens.refresh_token);
    let expires_at =
        chrono::Utc::now() + chrono::Duration::seconds(tokens.expires_in_seconds as i64);
    let refresh_expires_at =
        chrono::Utc::now() + chrono::Duration::seconds(tokens.refresh_expires_in_seconds as i64);

    session::create_session(
        pool,
        user_record.id,
        &access_hash,
        &refresh_hash,
        expires_at,
        refresh_expires_at,
    )
    .await
    .map_err(|_| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let response = LoginResponse {
        user: PublicUser::from(&user_record),
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: tokens.token_type,
        expires_in_seconds: tokens.expires_in_seconds,
        refresh_expires_in_seconds: tokens.refresh_expires_in_seconds,
    };

    Ok((StatusCode::OK, success(response)))
}

pub(super) async fn logout(headers: HeaderMap) -> Result<impl IntoResponse, ApiError> {
    let user = require_auth(&headers).await?;
    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let access_token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .unwrap_or_default();

    session::revoke_with_access_hash(pool, &hash_token(access_token))
        .await
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "DATABASE_UNAVAILABLE",
                "Database autentikasi tidak tersedia saat ini.",
            )
        })?;

    let _ = user;
    Ok((
        StatusCode::OK,
        success(LogoutResponse {
            message: "Sesi berhasil dicabut.",
        }),
    ))
}
