//
// landing.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use tide::{
  Server,
  Request,
  Result,
  Body
};

use tide_tera::prelude::*;
use crate::render::render;
use crate::state::State;

async fn index(_req:Request<State>) -> Result<Body> {
  render("index.html", &context!{})
}

pub fn route(app: &mut Server<State>) {
  let mut rt = app.at("/");

  rt.get(index);
}