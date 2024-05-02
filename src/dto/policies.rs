use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow, Validate)]
pub struct PolicyInput {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Policy {
    pub id: String,
    pub ttl: i32,
    pub content: String,
    pub search_tags: serde_json::Value,
    pub created_ts: String,
    pub updated_ts: String,
}
