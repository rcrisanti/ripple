use actix_web::{HttpResponse, ResponseError};
use std::fmt::Display;

#[derive(Debug)]
pub enum RippleError {
    UserError(String),
    EnvironmentError,
    R2D2Error,
    DieselError,
    ArgonauticaError,
    TeraError,
}

impl Display for RippleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RippleError::UserError(err) => write!(f, "UserError: {}", err),
            RippleError::EnvironmentError => write!(f, "EnvironmentError"),
            RippleError::R2D2Error => write!(f, "R2D2Error"),
            RippleError::DieselError => write!(f, "DieselError"),
            RippleError::ArgonauticaError => write!(f, "ArgonauticaError"),
            RippleError::TeraError => write!(f, "TeraError"),
        }
    }
}

impl ResponseError for RippleError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            RippleError::UserError(err) => HttpResponse::InternalServerError().json(err),
            RippleError::EnvironmentError => {
                HttpResponse::InternalServerError().json("EnvironmentError")
            }
            RippleError::R2D2Error => HttpResponse::InternalServerError().json("R2D2Error"),
            RippleError::DieselError => HttpResponse::InternalServerError().json("DieselError"),
            RippleError::ArgonauticaError => {
                HttpResponse::InternalServerError().json("ArgonauticaError")
            }
            RippleError::TeraError => HttpResponse::InternalServerError().json("TeraError"),
        }
    }
}

impl From<std::env::VarError> for RippleError {
    fn from(err: std::env::VarError) -> Self {
        log::error!("{:?}", err);
        RippleError::EnvironmentError
    }
}

impl From<r2d2::Error> for RippleError {
    fn from(err: r2d2::Error) -> Self {
        log::error!("{:?}", err);
        RippleError::R2D2Error
    }
}

impl From<diesel::result::Error> for RippleError {
    fn from(err: diesel::result::Error) -> Self {
        log::error!("{:?}", err);
        match err {
            diesel::result::Error::NotFound => {
                RippleError::UserError("Database request not found".to_string())
            }
            diesel::result::Error::DatabaseError(_kind, _info) => {
                RippleError::UserError("Database error".to_string())
            }
            _ => RippleError::DieselError,
        }
    }
}

impl From<argonautica::Error> for RippleError {
    fn from(err: argonautica::Error) -> Self {
        log::error!("{:?}", err);
        RippleError::ArgonauticaError
    }
}

impl From<tera::Error> for RippleError {
    fn from(err: tera::Error) -> Self {
        log::error!("{:?}", err);
        RippleError::TeraError
    }
}
