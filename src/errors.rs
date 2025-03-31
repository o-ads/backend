use axum::{
    Json,
    response::{IntoResponse, Response},
};
use http::status::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    ok: bool,
    message: String,
}
impl ErrorResponse {
    fn new(message: String) -> Self {
        Self { ok: false, message }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    NoRowReturned { entity_name: String },
    InvalidRequest,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NoRowReturned { entity_name } => {
                let message = format!("no {entity_name} found");
                (StatusCode::NOT_FOUND, Json(ErrorResponse::new(message))).into_response()
            }
            AppError::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::new(String::from("invalid request"))),
            )
                .into_response(),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(String::from("unexpected error"))),
            )
                .into_response(),
        }
    }
}
