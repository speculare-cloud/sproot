use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AppErrorType {
    ActixError,
    DieselError,
    NotFound,
    PoolError,
    InvalidRequest,
    InvalidToken,
    BlockingError,
    ServerError,
    UUIDError,
    SerdeError,
    InvalidJwtError,
    ExpiredJwtError,
    OtherJwtError,
    WalkDirError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            error_type: AppErrorType::ServerError,
        }
    }

    pub fn message(&self) -> String {
        match &*self {
            AppError {
                error_type: AppErrorType::NotFound,
                ..
            } => "The requested resource doesn't exists.".to_owned(),
            AppError {
                error_type: AppErrorType::PoolError,
                ..
            } => "Cannot get a connection from the R2D2's pool".to_owned(),
            AppError {
                error_type: AppErrorType::InvalidToken,
                ..
            } => "The token is invalid or has expired".to_owned(),
            AppError {
                error_type: AppErrorType::InvalidRequest,
                ..
            } => "Invalid request".to_owned(),
            AppError {
                error_type: AppErrorType::ServerError,
                ..
            } => "Server error, try again later".to_owned(),
            AppError { message, .. } => message.to_owned(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::InvalidRequest | AppErrorType::UUIDError | AppErrorType::SerdeError => {
                StatusCode::BAD_REQUEST
            }
            AppErrorType::InvalidToken
            | AppErrorType::ExpiredJwtError
            | AppErrorType::InvalidJwtError => StatusCode::UNAUTHORIZED,
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            error: self.message(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> AppError {
        AppError {
            message: format!("{}", error),
            error_type: AppErrorType::ServerError,
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> AppError {
        AppError {
            message: format!("{}", error),
            error_type: AppErrorType::PoolError,
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        let error_type = match error {
            diesel::result::Error::NotFound => AppErrorType::NotFound,
            _ => AppErrorType::DieselError,
        };

        AppError {
            message: format!("{}", error),
            error_type,
        }
    }
}

impl From<actix_web::Error> for AppError {
    fn from(error: actix_web::Error) -> AppError {
        AppError {
            message: format!("{}", error),
            error_type: AppErrorType::ActixError,
        }
    }
}

impl From<actix_web::error::BlockingError> for AppError {
    fn from(error: actix_web::error::BlockingError) -> AppError {
        AppError {
            message: format!("{}", error),
            error_type: AppErrorType::BlockingError,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> AppError {
        let error_type = match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppErrorType::ExpiredJwtError,
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidKeyFormat
            | jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName
            | jsonwebtoken::errors::ErrorKind::ImmatureSignature
            | jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => AppErrorType::InvalidJwtError,
            _ => AppErrorType::OtherJwtError,
        };

        AppError {
            message: format!("{}", error),
            error_type,
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::UUIDError,
        }
    }
}

impl From<askama::Error> for AppError {
    fn from(error: askama::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::ServerError,
        }
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(error: serde_json::error::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::SerdeError,
        }
    }
}

impl From<walkdir::Error> for AppError {
    fn from(error: walkdir::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::WalkDirError,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::WalkDirError,
        }
    }
}

impl From<simd_json::Error> for AppError {
    fn from(error: simd_json::Error) -> AppError {
        AppError {
            message: format!("{:?}", error),
            error_type: AppErrorType::SerdeError,
        }
    }
}
