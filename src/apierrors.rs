use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::DatabaseErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("the data for key `{0}` is not available")]
    ActixError(String),

    #[error("template error: failed to build")]
    AskamaError(#[from] askama::Error),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error("database connection error")]
    R2D2Error(#[from] r2d2::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    RustTlsError(#[from] rustls::Error),

    #[error("cannot en.de.code the content needed for the request")]
    SerdeError(#[from] serde_json::Error),

    #[error("cannot en.de.code the content needed for the request")]
    SimdError(#[from] simd_json::Error),

    #[error(transparent)]
    WalkDirError(#[from] walkdir::Error),

    #[error("server error: `{0}`")]
    ServerError(String),

    #[error("resource not found: `{0}`")]
    NotFoundError(String),

    #[error("authorization error: `{0}`")]
    AuthorizationError(String),

    #[error("invalid request: `{0}`")]
    InvalidRequestError(String),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error("invalid session: `{0}`")]
    SessionError(String),

    #[error("threading exception")]
    ActixBlockingError(#[from] actix_web::error::BlockingError),

    #[error("{0}")]
    ExplicitError(String),
}

impl From<ApiError> for actix_web::error::Error {
    fn from(err: ApiError) -> actix_web::error::Error {
        match err {
            ApiError::ExplicitError(desc) => actix_web::error::ErrorBadRequest(desc),
            ApiError::InvalidRequestError(_) | ApiError::SerdeError(_) | ApiError::SimdError(_) => {
                actix_web::error::ErrorBadRequest(err)
            }
            ApiError::DieselError(diesel::result::Error::NotFound) | ApiError::NotFoundError(_) => {
                actix_web::error::ErrorNotFound(String::from("the resource doesn't exists"))
            }
            ApiError::DieselError(diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            )) => actix_web::error::ErrorBadRequest(String::from("the resource already exists")),
            ApiError::SessionError(_) | ApiError::AuthorizationError(_) => {
                actix_web::error::ErrorUnauthorized(String::from(
                    "protected resource, you are not authorized",
                ))
            }
            _ => {
                error!("logging details of actix_error: {}", err);
                actix_web::error::ErrorInternalServerError(String::from("server error"))
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ExplicitError(desc) => (StatusCode::BAD_REQUEST, desc).into_response(),
            ApiError::InvalidRequestError(_) | ApiError::SerdeError(_) | ApiError::SimdError(_) => {
                (StatusCode::BAD_REQUEST, self).into_response()
            }
            ApiError::DieselError(diesel::result::Error::NotFound) | ApiError::NotFoundError(_) => {
                (StatusCode::NOT_FOUND, "the resource doesn't exists").into_response()
            }
            ApiError::DieselError(diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            )) => (StatusCode::BAD_REQUEST, "the resource already exists").into_response(),
            ApiError::SessionError(_) | ApiError::AuthorizationError(_) => (
                StatusCode::UNAUTHORIZED,
                "protected resource, you are not authorized",
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response(),
        }
    }
}
