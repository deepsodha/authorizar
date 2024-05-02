use crate::cedar::api::{fetch_entities, fetch_policies, prepare_cedar_request};
use crate::core::structs::{AuthorizationRequest, AuthorizationResponse};
use actix_web::{web, Responder};
use cedar_policy::{Authorizer, Response};

pub async fn authorize(authz: web::Json<AuthorizationRequest>) -> impl Responder {
    let authz_call = tokio::try_join!(
        prepare_cedar_request(&authz),
        fetch_policies(&authz),
        fetch_entities(&authz)
    );

    let authz_response: Option<Response> = match authz_call {
        Ok((req, pol, ent)) => Some(Authorizer::new().is_authorized(&req, &pol, &ent)),
        Err(_err) => None,
    };

    return match authz_response {
        Some(r) => web::Json(AuthorizationResponse::authz_decision(
            r.decision(),
            r.diagnostics().clone(),
        )),
        None => web::Json(AuthorizationResponse::deny()),
    };
}
