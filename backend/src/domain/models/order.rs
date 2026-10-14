use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Order {
    pub id: String,
    pub service_id: String,
    pub status: OrderStatus,
    pub scheduled_at: String,
    pub address: String,
    pub price_rupiah: u64,
    pub customer_service_fee_rupiah: u64,
    pub worker_service_fee_rupiah: u64,
    pub simulated: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub service_category_id: Option<String>,
    pub address_id: Option<String>,
    pub service_id: Option<String>,
    pub scheduled_at: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

impl CreateOrderRequest {
    pub fn is_valid(&self) -> bool {
        let has_service = self
            .service_category_id
            .as_deref()
            .map(is_valid_uuid)
            .unwrap_or(self.service_id.as_deref().is_some());

        let has_address = self.address_id.as_deref().map(is_valid_uuid).unwrap_or(
            self.address
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty()),
        );

        let scheduled_ok = self
            .scheduled_at
            .as_deref()
            .map(|value| !value.trim().is_empty() && value.chars().count() <= 64)
            .unwrap_or(true);

        has_service
            && has_address
            && scheduled_ok
            && self
                .notes
                .as_deref()
                .is_none_or(|notes| notes.chars().count() <= 2_000)
    }
}

fn is_valid_uuid(value: &str) -> bool {
    Uuid::parse_str(value.trim()).is_ok()
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Accepted,
    InProgress,
    Completed,
    Cancelled,
}

impl OrderStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "accepted" => Some(Self::Accepted),
            "in_progress" => Some(Self::InProgress),
            "completed" => Some(Self::Completed),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Accepted)
                | (Self::Pending, Self::Cancelled)
                | (Self::Accepted, Self::InProgress)
                | (Self::Accepted, Self::Cancelled)
                | (Self::InProgress, Self::Completed)
                | (Self::InProgress, Self::Cancelled)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::OrderStatus;

    #[test]
    fn order_status_only_allows_documented_transitions() {
        assert!(OrderStatus::Pending.can_transition_to(OrderStatus::Accepted));
        assert!(OrderStatus::Accepted.can_transition_to(OrderStatus::InProgress));
        assert!(OrderStatus::InProgress.can_transition_to(OrderStatus::Completed));
        assert!(OrderStatus::Pending.can_transition_to(OrderStatus::Cancelled));
        assert!(!OrderStatus::Completed.can_transition_to(OrderStatus::Cancelled));
        assert!(!OrderStatus::Cancelled.can_transition_to(OrderStatus::Accepted));
        assert!(!OrderStatus::Pending.can_transition_to(OrderStatus::Completed));
    }
}
