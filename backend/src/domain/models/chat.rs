use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub body: String,
}

impl CreateMessageRequest {
    pub fn is_valid(&self) -> bool {
        let trimmed = self.body.trim();
        !trimmed.is_empty() && trimmed.chars().count() <= 4_000
    }
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct Conversation {
    pub id: String,
    pub order_id: String,
    pub created_at: String,
    pub updated_at: String,
}
