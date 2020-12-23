//
// connection.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use mongodb::{
  options::ClientOptions,
  Client,
  Database
};

use crate::env::Env;

pub struct Connection {
  pub client:Client,
  pub db:Database
}

impl Connection {
  pub async fn connect() -> tide::Result<Self> {
    let env = Env::load();
    let url = env.db.url;
    let name = &env.db.name;

    let mut options = ClientOptions::parse(&url).await?;
    options.app_name  = Some(name.to_string());

    let client = Client::with_options(options)?;
    let db = client.database(name);

    Ok(Self{
      client: client,
      db: db
    })
  }
}