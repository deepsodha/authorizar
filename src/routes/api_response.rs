use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiResponse {
    pub status_code: String,
    pub message: String,
    pub data: serde_json::Value,
}
