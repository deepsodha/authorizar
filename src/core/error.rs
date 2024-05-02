use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationRequestError {
    #[error("failed to parse principal")]
    InvalidPrincipal,
    #[error("failed to parse action")]
    InvalidAction,
    #[error("failed to parse resource")]
    InvalidResource,
    #[error("failed to parse context")]
    InvalidContext,
    #[error("failed to parse entities")]
    InvalidEntities,
    #[error("failed to parse policies")]
    InvalidPolicies,
}
