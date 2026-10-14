use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub r#type: String,
    pub title: String,
    pub body: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub read_at: Option<String>,
    pub created_at: String,
}
