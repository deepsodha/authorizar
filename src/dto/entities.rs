use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct EntityInput {
    pub uid: UID,
    pub attrs: HashMap<String, serde_json::Value>,
    pub parents: Vec<Parent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct UID {
    pub r#type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Parent {
    pub r#type: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Entity {
    pub id: String,
    pub eid: String,
    pub etype: String,
    pub content: serde_json::Value,
    pub search_tags: String,
    pub created_ts: String,
    pub updated_ts: String,
}
