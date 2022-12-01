//! Logging for _trace_ logs

use tracing_subscriber::{
  fmt::format::{Format, JsonFields},
  prelude::*,
  EnvFilter, Registry,
};

use crate::{SystemError, SystemResult};

mod middleware;
pub use middleware::{
  LoggerMiddleware,
  LoggerLayer,
};

pub struct Logger {
  is_prod: bool,
  level: String,
  path: String,
  filename: String,
}

impl Logger {
  pub fn init(env: &str, path: &str, filename: &str) -> SystemResult<Self> {
    let (is_prod, level) = match env {
      "production" => (true, string!("error")),
      _ => (false, string!("info")),
    };

    let path = path.to_string();
    let filename = filename.to_string();

    Ok(Self {
      is_prod,
      level,
      path,
      filename,
    })
  }

  fn format(&self) -> Format {
    tracing_subscriber::fmt::format()
      .with_level(true) // don't include levels in formatted output
      .with_target(true) // don't include targets
      .with_thread_ids(true) // include the thread ID of the current thread
      .with_thread_names(true) // include the name of the current thread
      .with_file(true)
      .with_line_number(true)
  }

  fn filter(&self) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.level))
  }

  pub fn subscribe(&self) -> SystemResult<()> {
    let formatter = self.format();
    let filter = self.filter();

    if self.is_prod {
      let file_appender = tracing_appender::rolling::daily(&self.path, &self.filename);

      let layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .event_format(formatter.json())
        .fmt_fields(JsonFields::new())
        .with_writer(file_appender);

      let subscriber = Registry::default().with(filter).with(layer);

      tracing::subscriber::set_global_default(subscriber)
        .map_err(|err| SystemError::ConfigError(err.to_string()))?;
    } else {
      let layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .event_format(formatter.pretty())
        .with_writer(std::io::stdout);

      let subscriber = Registry::default().with(filter).with(layer);

      tracing::subscriber::set_global_default(subscriber)
        .map_err(|err| SystemError::ConfigError(err.to_string()))?;
    }
    Ok(())
  }
}
