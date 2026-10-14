use std::env;

use axum::http::{HeaderMap, StatusCode};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use uuid::Uuid;

use crate::{
    core::{api_response::ApiError, app_state::AppState},
    infrastructure::repositories::user::{find_by_id, UserRecord},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClaims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
    pub typ: String,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: String,
    pub display_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub display_name: String,
    pub role: String,
}

impl From<&UserRecord> for PublicUser {
    fn from(user: &UserRecord) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            display_name: user.display_name.clone(),
            role: user.role.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in_seconds: usize,
    pub refresh_expires_in_seconds: usize,
}

fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "development-secret-change-me".to_string())
}

fn access_token_ttl_seconds() -> usize {
    env::var("JWT_ACCESS_TTL_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3600)
}

fn refresh_token_ttl_seconds() -> usize {
    env::var("JWT_REFRESH_TTL_SECONDS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(7_200)
}

fn validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iat", "sub", "role", "typ"]);
    validation
}

pub fn normalize_email(raw: &str) -> Option<String> {
    let email = raw.trim();
    if email.is_empty() {
        return None;
    }
    let lowered = email.to_ascii_lowercase();
    if lowered.contains(char::is_whitespace) || lowered.split_once('@').is_none() {
        return None;
    }
    Some(lowered)
}

pub fn normalize_phone(raw: &str) -> Option<String> {
    let phone = raw.trim();
    if phone.is_empty() {
        return None;
    }
    let digits: String = phone.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.len() < 8 || digits.len() > 15 {
        return None;
    }
    Some(digits)
}

pub fn require_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> Result<(), ApiError> {
    if allowed_roles.iter().any(|role| *role == user.role) {
        return Ok(());
    }

    Err(ApiError::new(
        StatusCode::FORBIDDEN,
        "FORBIDDEN",
        "Anda tidak memiliki izin untuk melakukan operasi ini.",
    ))
}

pub async fn require_auth(headers: &HeaderMap) -> Result<AuthenticatedUser, ApiError> {
    let token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Autentikasi diperlukan untuk mengakses endpoint ini.",
            )
        })?;

    let claims = decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &validation(),
    )
    .map(|data| data.claims)
    .map_err(|_| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_TOKEN",
            "Token tidak valid atau kedaluwarsa.",
        )
    })?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_TOKEN",
            "Token tidak valid atau kedaluwarsa.",
        )
    })?;

    let state = AppState::current();
    let pool = state.database.as_ref().ok_or_else(|| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    let user = find_by_id(pool, user_id)
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
                "INVALID_TOKEN",
                "Token tidak valid atau kedaluwarsa.",
            )
        })?;

    if !user.is_active {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "USER_DISABLED",
            "Akun pengguna tidak aktif.",
        ));
    }

    if user.role != claims.role {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "INVALID_TOKEN",
            "Token tidak valid untuk peran pengguna ini.",
        ));
    }

    let access_hash = format!("{:x}", sha2::Sha256::digest(token.as_bytes()));
    let session = crate::infrastructure::repositories::session::find_active_session_for_user(
        pool,
        user_id,
        &access_hash,
    )
    .await
    .map_err(|_| {
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "DATABASE_UNAVAILABLE",
            "Database autentikasi tidak tersedia saat ini.",
        )
    })?;

    if session.is_none() {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "SESSION_EXPIRED",
            "Sesi login Anda sudah berakhir atau dicabut.",
        ));
    }

    Ok(AuthenticatedUser {
        id: user.id,
        role: user.role,
        display_name: user.display_name,
        email: user.email,
        phone: user.phone,
    })
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    use argon2::{Argon2, PasswordHasher};
    use rand::rngs::OsRng;

    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
    let hasher = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(65536, 3, 1, Some(32)).map_err(|_| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUTHENTICATION_ERROR",
                "Pengaturan hash kata sandi tidak valid.",
            )
        })?,
    );

    hasher
        .hash_password(password.as_bytes(), &salt)
        .map(|hashed| hashed.to_string())
        .map_err(|_| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUTHENTICATION_ERROR",
                "Pengaturan hash kata sandi tidak valid.",
            )
        })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let parsed = PasswordHash::new(hash).map_err(|_| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "AUTHENTICATION_ERROR",
            "Hash kata sandi tidak dapat dibaca.",
        )
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn generate_tokens(user: &UserRecord) -> Result<AuthTokens, ApiError> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let access_exp =
        (now + chrono::Duration::seconds(access_token_ttl_seconds() as i64)).timestamp() as usize;
    let refresh_exp =
        (now + chrono::Duration::seconds(refresh_token_ttl_seconds() as i64)).timestamp() as usize;

    let access_claims = AuthClaims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp: access_exp,
        iat,
        typ: "access".to_string(),
    };
    let refresh_claims = AuthClaims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp: refresh_exp,
        iat,
        typ: "refresh".to_string(),
    };

    let access_token = encode(
        &Header::new(Algorithm::HS256),
        &access_claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|_| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "AUTHENTICATION_ERROR",
            "Token akses tidak dapat dibuat.",
        )
    })?;

    let refresh_token = encode(
        &Header::new(Algorithm::HS256),
        &refresh_claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|_| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "AUTHENTICATION_ERROR",
            "Token refresh tidak dapat dibuat.",
        )
    })?;

    Ok(AuthTokens {
        access_token,
        refresh_token,
        token_type: "Bearer",
        expires_in_seconds: access_token_ttl_seconds(),
        refresh_expires_in_seconds: refresh_token_ttl_seconds(),
    })
}

#[cfg(test)]
mod tests {
    use super::{require_role, AuthenticatedUser};
    use uuid::Uuid;

    #[test]
    fn require_role_accepts_allowed_role() {
        let user = AuthenticatedUser {
            id: Uuid::nil(),
            role: "worker".to_string(),
            display_name: "Test Worker".to_string(),
            email: None,
            phone: None,
        };

        assert!(require_role(&user, &["worker", "customer"]).is_ok());
    }

    #[test]
    fn require_role_rejects_disallowed_role() {
        let user = AuthenticatedUser {
            id: Uuid::nil(),
            role: "customer".to_string(),
            display_name: "Test Customer".to_string(),
            email: None,
            phone: None,
        };

        assert!(require_role(&user, &["worker"]).is_err());
    }
}
