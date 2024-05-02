use cedar_policy::{Decision, Diagnostics, Entity};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct AuthorizationRequest {
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub context: Option<Value>,
    pub entities: Option<Value>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationResponse {
    pub decision: Decision,
    pub diagnostics: Option<Diagnostics>,
}

impl AuthorizationResponse {
    pub fn deny() -> Self {
        return Self {
            decision: Decision::Deny,
            diagnostics: None,
        };
    }

    pub fn authz_decision(decision: Decision, diagnostics: Diagnostics) -> Self {
        return Self {
            decision,
            diagnostics: Some(diagnostics),
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct Policy {
    pub id: String,
    pub content: String,
    pub search_tags: Vec<String>,
    pub ttl: usize,
}
