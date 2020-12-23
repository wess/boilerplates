//
// env.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use std::env as stdenv;

pub struct Database {
  pub url:  String,
  pub name: String
}

pub struct Env {
  pub asset_path:String,
  pub db:Database,
}

impl Env {
  pub fn load() -> Self {
    return Env{
      asset_path: stdenv::var("ASSET_PATH").unwrap_or_default(),
      db: Database{
        url: stdenv::var("MONGO_URL").unwrap_or_default(),
        name: stdenv::var("DB_NAME").unwrap_or_default(),
      }
    }
  }
}
