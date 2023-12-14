use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub status_code: u16,
    pub message: String,
}

impl AppError {
    pub fn new(status_code: u16, message: String) -> AppError {
        AppError {
            status_code,
            message,
        }
    }

    pub fn new_internal_server_error(message: String) -> AppError {
        AppError::new(500, message)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::DatabaseError(_, err) => {
                AppError::new(StatusCode::CONFLICT.as_u16(), err.message().to_string())
            }
            DieselError::NotFound => AppError::new(
                StatusCode::NOT_FOUND.as_u16(),
                "The 'turtle' record not found".to_string(),
            ),
            err => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                format!("Unknown Diesel error: {}", err),
            ),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < StatusCode::CONFLICT.as_u16() {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
