use std::error::Error;
use std::{str::FromStr, time::Duration};

use cedar_policy::{Context, Entities, EntityUid, PolicySet, Request};

use crate::core::{error::AuthorizationRequestError, structs::AuthorizationRequest};

pub async fn prepare_cedar_request(
    authz_request: &AuthorizationRequest,
) -> Result<Request, Box<dyn Error + Send + Sync>> {
    let principal = match EntityUid::from_str(&authz_request.principal) {
        Ok(p) => p,
        Err(_) => return Err(Box::from(AuthorizationRequestError::InvalidPrincipal)),
    };

    let action = match EntityUid::from_str(&authz_request.action) {
        Ok(a) => a,
        Err(_) => return Err(Box::from(AuthorizationRequestError::InvalidAction)),
    };

    let resource = match EntityUid::from_str(&authz_request.resource) {
        Ok(r) => r,
        Err(_) => return Err(Box::from(AuthorizationRequestError::InvalidResource)),
    };

    let context = match &authz_request.context {
        Some(c) => match Context::from_json_value(c.clone(), None) {
            Ok(c) => c,
            Err(_err) => return Err(Box::from(AuthorizationRequestError::InvalidContext)),
        },
        _ => Context::empty(),
    };

    return Ok(Request::new(
        Some(principal),
        Some(action),
        Some(resource),
        context,
    ));
}

pub async fn fetch_entities(
    _authz_request: &AuthorizationRequest,
) -> Result<Entities, Box<dyn Error + Send + Sync>> {
    // check if entities are present in authz_request
    // if not present, fetch entities from sqlx

    let ents = r#"[
                {
                    "uid": {"type":"User","id":"alice"},
                    "attrs": {
                        "age":19,
                        "ip_addr":{"__extn":{"fn":"ip", "arg":"10.0.1.101"}}
                    },
                    "parents": []
                }
            ]"#;

    return match Entities::from_json_str(ents, None) {
        Ok(e) => Ok(e),
        Err(_) => Err(Box::from(AuthorizationRequestError::InvalidEntities)),
    };
}

pub async fn fetch_policies(
    _authz_request: &AuthorizationRequest,
) -> Result<PolicySet, Box<dyn Error + Send + Sync>> {
    // fetch policies from sqlx

    let s = r#"                       
                        permit (
                        principal == User::"alice",
                        action == Action::"view",
                        resource == Album::"trip"
                        )
                        when { principal.ip_addr.isIpv4() };

                        permit (
                            principal == User::"alice",
                            action == Action::"update",
                            resource == Album::"trip"
                            )
                            when { principal.ip_addr.isIpv4() };
                        "#;
    return match PolicySet::from_str(s) {
        Ok(p) => Ok(p),
        Err(_err) => Err(Box::from(AuthorizationRequestError::InvalidPolicies)),
    };
}
