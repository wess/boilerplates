use std::env;
use dotenvy::dotenv;

pub struct Server {
  pub host:String,
  pub port:u32,
}

impl Default for Server {
  fn default() -> Self {
    dotenv().ok();

    let host = 
      env::var("SERVER_HOST")
      .unwrap_or(string!("127.0.0.1"));

    let port = 
      env::var("SERVER_PORT")
      .unwrap_or(string!("3000"))
      .parse::<u32>()
      .unwrap();

    Self {
      host,
      port,
    }
  }
}

pub(crate) struct Env {
  pub server:Server,
}

impl Default for Env {
  fn default() -> Self {
    Self {
      server:Server::default(),
    }
  }
}