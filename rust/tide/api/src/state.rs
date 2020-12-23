//
// state.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use async_std::sync::Arc;

use crate::data::connection::Connection;
use crate::env::Env;

#[derive(Clone)]
pub struct State {
  pub env:Arc<Env>,
  pub db:Arc<Connection>
}

impl State {
  pub async fn init() -> Self {
    let conn = match Connection::connect().await {
      Ok(c) => c,
      Err(_) => panic!("Unable to connect to mongodb")
    };

    let env = Env::load();

    Self {
      env: Arc::new(env),
      db: Arc::new(conn)
    }
  }
}