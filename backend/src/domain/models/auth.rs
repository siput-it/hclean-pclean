use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Customer,
    Worker,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub role: UserRole,
}

impl RegisterRequest {
    pub fn is_valid(&self) -> bool {
        valid_name(&self.name)
            && valid_password(&self.password)
            && matches!(self.role, UserRole::Customer | UserRole::Worker)
            && (self.email.as_ref().is_some_and(|value| valid_email(value))
                || self
                    .phone_number
                    .as_ref()
                    .is_some_and(|value| valid_phone(value)))
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
}

impl LoginRequest {
    pub fn is_valid(&self) -> bool {
        valid_password(&self.password)
            && (self.email.as_ref().is_some_and(|value| valid_email(value))
                || self
                    .phone_number
                    .as_ref()
                    .is_some_and(|value| valid_phone(value)))
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

impl UpdateProfileRequest {
    pub fn is_valid(&self) -> bool {
        let has_field = self.name.is_some() || self.phone_number.is_some();
        has_field
            && self.name.as_ref().is_none_or(|name| valid_name(name))
            && self
                .phone_number
                .as_ref()
                .is_none_or(|phone| valid_phone(phone))
    }
}

fn valid_name(name: &str) -> bool {
    let name = name.trim();
    !name.is_empty() && name.chars().count() <= 100
}

fn valid_email(email: &str) -> bool {
    let email = email.trim();
    !email.is_empty()
        && !email.chars().any(char::is_whitespace)
        && email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
        })
}

fn valid_phone(phone: &str) -> bool {
    let phone = phone.trim();
    !phone.is_empty()
        && phone
            .chars()
            .all(|ch| ch.is_ascii_digit() || matches!(ch, '+' | ' ' | '-' | '(' | ')'))
        && phone.chars().filter(|ch| ch.is_ascii_digit()).count() >= 8
        && phone.chars().filter(|ch| ch.is_ascii_digit()).count() <= 15
}

fn valid_password(password: &str) -> bool {
    password.chars().count() >= 8
}
