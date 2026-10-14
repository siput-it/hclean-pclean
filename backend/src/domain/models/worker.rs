use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WorkerProfileUpdateRequest {
    pub bio: Option<String>,
    pub is_available: Option<bool>,
}

impl WorkerProfileUpdateRequest {
    pub fn is_valid(&self) -> bool {
        let has_value = self.bio.is_some() || self.is_available.is_some();
        has_value && self.bio.as_deref().is_none_or(valid_bio) && self.is_available.is_some()
    }
}

#[derive(Debug, Serialize)]
pub struct WorkerProfileResponse {
    pub user_id: String,
    pub bio: Option<String>,
    pub is_available: bool,
}

fn valid_bio(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.chars().count() <= 1000
}
