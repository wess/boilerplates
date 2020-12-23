//
// main.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

extern crate serde_json;
extern crate tera;

pub mod env;
pub mod data;
pub mod state;
pub mod routes;
pub mod render;

#[async_std::main]
async fn main() -> std::io::Result<()> {
  tide::log::start();

  let state = state::State::init().await;
  let mut app = tide::with_state(state);

  app.at("/public").serve_dir("public/")?;

  routes::landing::route(&mut app);

  app.listen("127.0.0.1:3000").await?;

  Ok(())
}
