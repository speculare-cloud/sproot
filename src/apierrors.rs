use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::DatabaseErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
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

    #[error("server error: `{0:?}`")]
    ServerError(Option<String>),

    #[error("resource not found: `{0:?}`")]
    NotFoundError(Option<String>),

    #[error("authorization error: `{0:?}`")]
    AuthorizationError(Option<String>),

    #[error("invalid request: `{0:?}`")]
    InvalidRequestError(Option<String>),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error("invalid session: `{0:?}`")]
    SessionError(Option<String>),

    #[error("invalid session: `{0:?}`")]
    ActixSessionError(#[from] actix_session::SessionGetError),

    #[error("invalid session: `{0:?}`")]
    ActixSetSessionError(#[from] actix_session::SessionInsertError),

    #[error("threading exception")]
    ActixBlockingError(#[from] actix_web::error::BlockingError),

    #[error("{0}")]
    ExplicitError(String),
}

impl From<ApiError> for actix_web::error::Error {
    fn from(err: ApiError) -> actix_web::error::Error {
        match err {
            ApiError::ExplicitError(desc) => actix_web::error::ErrorBadRequest(desc),
            ApiError::InvalidRequestError(x) => actix_web::error::ErrorBadRequest(
                x.unwrap_or_else(|| String::from("invalid request")),
            ),
            ApiError::SerdeError(_) | ApiError::SimdError(_) => {
                actix_web::error::ErrorBadRequest(err)
            }
            ApiError::NotFoundError(x) => actix_web::error::ErrorNotFound(
                x.unwrap_or_else(|| String::from("the resource doesn't exists")),
            ),
            ApiError::DieselError(diesel::result::Error::NotFound) => {
                actix_web::error::ErrorNotFound(String::from("the resource doesn't exists"))
            }
            ApiError::DieselError(diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            )) => actix_web::error::ErrorBadRequest(String::from("the resource already exists")),
            ApiError::SessionError(x) | ApiError::AuthorizationError(x) => {
                actix_web::error::ErrorUnauthorized(
                    x.unwrap_or_else(|| String::from("protected resource, you are not authorized")),
                )
            }
            ApiError::ActixSessionError(_) | ApiError::ActixSetSessionError(_) => {
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
            ApiError::InvalidRequestError(x) => (
                StatusCode::BAD_REQUEST,
                x.unwrap_or_else(|| String::from("invalid request")),
            )
                .into_response(),
            ApiError::SerdeError(_) | ApiError::SimdError(_) => {
                (StatusCode::BAD_REQUEST, "").into_response()
            }
            ApiError::NotFoundError(x) => (
                StatusCode::NOT_FOUND,
                x.unwrap_or_else(|| String::from("the resource doesn't exists")),
            )
                .into_response(),
            ApiError::DieselError(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "the resource doesn't exists").into_response()
            }
            ApiError::DieselError(diesel::result::Error::DatabaseError(
                DatabaseErrorKind::UniqueViolation,
                _,
            )) => (StatusCode::BAD_REQUEST, "the resource already exists").into_response(),
            ApiError::SessionError(x) | ApiError::AuthorizationError(x) => (
                StatusCode::UNAUTHORIZED,
                x.unwrap_or_else(|| String::from("protected resource, you are not authorized")),
            )
                .into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response(),
        }
    }
}
