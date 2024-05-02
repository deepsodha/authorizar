use actix_web::http::StatusCode;
use actix_web::ResponseError;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    // 403
    #[error("Forbidden: {0}")]
    _Forbidden(String),

    // 404
    #[error("Not Found: {0}")]
    NotFound(String),

    // 422
    #[error("{0}")]
    Validation(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    StdError(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::_Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::StdError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let res = errors
            .field_errors()
            .iter()
            .map(|(_, &y)| {
                y.iter().fold("".into(), |a: String, b| {
                    a + b.clone().message.get_or_insert("".into())
                })
            })
            .collect();

        ApiError::Validation(res)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        ApiError::NotFound(value.to_string())
    }
}
