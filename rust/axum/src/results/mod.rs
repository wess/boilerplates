use axum::response::{
  Html,
  Json,
  Redirect
};

mod errors;
pub use errors::{
  ServerError,
  SystemError
};

pub type ServerResult<T> = anyhow::Result<T, ServerError>;
pub type SystemResult<T> = anyhow::Result<T, SystemError>;

pub type TextResult = ServerResult<String>;
pub type HTMLResult<T> = ServerResult<Html<T>>;
pub type JSONResult<T> = ServerResult<Json<T>>;
pub type RedirectResult = ServerResult<Redirect>;
