//
// mod.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/09/2020
// 
// Copywrite (c) 2020 Wess.io
//

use lazy_static::lazy_static;

use tera::{
  Tera,
  Context
};

use tide_tera::prelude::*;

use tide::{
  Result,
  Body
};

lazy_static! {
  static ref RENDERER: Tera = {
      let mut tera = match Tera::new("src/views/**/*") {
          Ok(t) => t,
          Err(e) => {
              println!("Parsing error(s): {}", e);
              ::std::process::exit(1);
          }
      };

      tera.autoescape_on(vec!["html", ".sql"]);
      tera
  };
}

pub fn render(template:&str, context:&Context) -> Result<Body> {
  RENDERER.render_body(template, &context)
}