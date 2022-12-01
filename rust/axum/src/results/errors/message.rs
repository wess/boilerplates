use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage {
  pub code: u16,
  pub message: String,
}
