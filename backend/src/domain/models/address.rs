use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddressCreateRequest {
    pub label: Option<String>,
    pub recipient_name: String,
    pub recipient_phone: String,
    pub full_address: String,
    pub additional_details: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: Option<bool>,
}

impl AddressCreateRequest {
    pub fn is_valid(&self) -> bool {
        valid_label(self.label.as_deref())
            && valid_recipient_name(&self.recipient_name)
            && valid_phone(&self.recipient_phone)
            && valid_full_address(&self.full_address)
            && self
                .additional_details
                .as_deref()
                .is_none_or(valid_additional_details)
            && valid_coordinate_pair(self.latitude, self.longitude)
    }
}

#[derive(Debug, Deserialize)]
pub struct AddressUpdateRequest {
    pub label: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_phone: Option<String>,
    pub full_address: Option<String>,
    pub additional_details: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: Option<bool>,
}

impl AddressUpdateRequest {
    pub fn is_valid(&self) -> bool {
        let has_value = self.label.is_some()
            || self.recipient_name.is_some()
            || self.recipient_phone.is_some()
            || self.full_address.is_some()
            || self.additional_details.is_some()
            || self.latitude.is_some()
            || self.longitude.is_some()
            || self.is_primary.is_some();

        has_value
            && self
                .label
                .as_deref()
                .is_none_or(|value| valid_label(Some(value)))
            && self
                .recipient_name
                .as_deref()
                .is_none_or(valid_recipient_name)
            && self.recipient_phone.as_deref().is_none_or(valid_phone)
            && self.full_address.as_deref().is_none_or(valid_full_address)
            && self
                .additional_details
                .as_deref()
                .is_none_or(valid_additional_details)
            && valid_coordinate_pair(self.latitude, self.longitude)
    }
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub recipient_name: String,
    pub recipient_phone: String,
    pub full_address: String,
    pub additional_details: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

#[derive(Debug, Serialize)]
pub struct AddressDeleteResponse {
    pub message: &'static str,
}

fn valid_label(label: Option<&str>) -> bool {
    match label {
        Some(value) => {
            let trimmed = value.trim();
            !trimmed.is_empty() && trimmed.chars().count() <= 80
        }
        None => true,
    }
}

fn valid_recipient_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.chars().count() <= 120
}

fn valid_phone(phone: &str) -> bool {
    let digits: String = phone.chars().filter(|ch| ch.is_ascii_digit()).collect();
    !phone.trim().is_empty() && digits.len() >= 8 && digits.len() <= 15
}

fn valid_full_address(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().count() <= 500
}

fn valid_additional_details(value: &str) -> bool {
    value.chars().count() <= 300
}

fn valid_coordinate_pair(latitude: Option<f64>, longitude: Option<f64>) -> bool {
    match (latitude, longitude) {
        (Some(lat), Some(lon)) => (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon),
        (None, None) => true,
        _ => false,
    }
}
