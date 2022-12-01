//! System errors.

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SystemError {
  #[error("Panic: {0}")]
  Panic(String),

  #[error("Config error: {0}")]
  ConfigError(String),

  #[error("Database error: {0}")]
  DatabaseError(String),

  #[error("Redis error: {0}")]
  RedisError(String),

  #[error("CLI error: {0}")]
  Error(String),

  #[error("Server error: {0}")]
  ServerError(String),
}
