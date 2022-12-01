/***** REMOVE FOR PRODUCTION */
#![allow(dead_code)]
#![allow(unused_imports)]
/***** REMOVE FOR PRODUCTION */

#[macro_use]
extern crate oxide;

#[macro_use]
extern crate lazy_static;

extern crate chrono;

extern crate serde;

use std::net::SocketAddr;

use axum::{
  response::Html, 
  routing::get, 
  Router
};

use tower::ServiceBuilder;

mod results;
pub use results::{
  ServerResult,
  SystemResult,
  HTMLResult,
  JSONResult,
  RedirectResult,
  ServerError,
  SystemError,
};


mod env;
use env::Env;

mod logging;
use logging::Logger;

mod views;

mod livereload;
use livereload::LiveReloadLayer;

mod router;
mod styles;

pub async fn launch() -> SystemResult<()> {
    let env = Env::default();

    Logger::init(
      "dev",
      std::env::current_dir().unwrap().to_str().unwrap(),
      "logging.txt"
    )
    .unwrap()
    .subscribe()
    .unwrap();

    let app = 
      router::build()
        .await
        .unwrap()
        .layer(LiveReloadLayer::new())
        .layer(logging::LoggerLayer);

    // run it
    let addr = format!(
      "{}:{}",
      env.server.host,
      env.server.port
    )
    .parse::<SocketAddr>()
    .unwrap();

    console_info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
