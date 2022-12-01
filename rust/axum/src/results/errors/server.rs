//! Server specific errors.

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use derive_more::Display;
use serde_json::json;

use super::ErrorMessage;

#[derive(Display, Debug, thiserror::Error)]
pub enum ServerError {
  #[display(fmt = "{}", message)]
  InternalError { message: String },

  #[display(fmt = "{}", message)]
  BadRequest { message: String },

  #[display(fmt = "{}", message)]
  NotFound { message: String },

  #[display(fmt = "{}", message)]
  UnprocessableEntity { message: String },

  #[display(fmt = "Request Timeout")]
  Timeout,

  #[display(fmt = "Unauthorized")]
  Unauthorized,

  #[display(fmt = "Too Many Requests")]
  TooManyRequests,

  #[display(fmt = "Method Not Allowed")]
  MethodNotAllowed,
}

impl ServerError {
  pub fn name(&self) -> String {
    match self {
      Self::NotFound { message: m } => m.to_owned(),
      Self::BadRequest { message: m } => m.to_owned(),
      Self::InternalError { message: m } => m.to_owned(),
      Self::UnprocessableEntity { message: m } => m.to_owned(),
      Self::Unauthorized => "Unauthorized".to_owned(),
      Self::Timeout => "Request Timeout".to_owned(),
      Self::TooManyRequests => "Too Many Requests".to_owned(),
      Self::MethodNotAllowed => "Method Not Allowed".to_owned(),
    }
  }
}

impl IntoResponse for ServerError {
  fn into_response(self) -> Response {
    let status = match self {
      ServerError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
      ServerError::NotFound { .. } => StatusCode::NOT_FOUND,
      ServerError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
      ServerError::BadRequest { .. } => StatusCode::BAD_REQUEST,
      ServerError::Timeout { .. } => StatusCode::REQUEST_TIMEOUT,
      ServerError::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,
      ServerError::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
      ServerError::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
    };

    let body = Json(json!(ErrorMessage {
      code: status.as_u16(),
      message: self.to_string(),
    }));

    (status, body).into_response()
  }
}
