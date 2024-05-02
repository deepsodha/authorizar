use cedar_policy::{Entities, Entity};

use super::structs::AuthorizationRequest;

impl AuthorizationRequest {
    async fn calculate_entities(&self) -> Entities {
        // Grab attributes and roles for principal, and resource. This must be done concurrently.

        // Convert all entities fetched to serde_json::Value

        // Call Entities::from_json_value(json, None) to convert json value to entity

        todo!();
    }
}
