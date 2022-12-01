use std::collections::HashMap;
use lazy_static::lazy_static;

use axum::{
  response::Html,
};

use tera::{
  Tera,
  Context
};
use crate::ServerResult;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let path = format!("{}/src/views/templates/**/*", std::env!("CARGO_MANIFEST_DIR"));

    let mut tera = match Tera::new(&path) {
      Ok(t) => t,
      Err(e) => {
        println!("Parsing error(s): {}", e);
        std::process::exit(1);
      }
    };

    tera.autoescape_on(vec![]);

    tera
  };
}

pub async fn render(tpl:&str, params:Option<HashMap<String, String>>) -> ServerResult<Html<String>> {
  let mut context = Context::new();

  if let Some(params) = params {
    for (key, value) in params {
      context.insert(key, &value);
    }
  }

  Ok(Html(
    TEMPLATES
    .render(&tpl, &context)
    .unwrap()
  ))
}
