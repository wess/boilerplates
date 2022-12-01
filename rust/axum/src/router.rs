use std::{
  collections::HashSet,
  future::ready, 
  sync::Mutex,
  net::SocketAddr, 
  time::Duration,
};

use tokio::{
  signal,
  sync::broadcast,
};

use axum::{
  error_handling::HandleErrorLayer,
  middleware,
  http::StatusCode,
  extract::{
    Path,
  },
  routing::{
    get, 
    get_service,
  },
  response::IntoResponse,
  Extension, 
  Json,
  Router,
};

use tower::ServiceBuilder;

use tower_http::{
  services::{
    ServeDir,
    ServeFile,
  }, 
  ServiceBuilderExt, 
  cors::{
    CorsLayer,
    Any
  }, 
  trace::TraceLayer
};


use crate::{
  ServerResult, 
  SystemResult, 
};

use crate::styles;

pub async fn build() -> ServerResult<Router> {
  let styles = styles::router(None).await;

  let cors = 
    CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);

  let router = 
    Router::new()
    .nest("/styles", styles)
    .merge(static_router())
    .layer(cors);

  Ok(router)
}

fn static_router() -> Router {
  let serve_dir = ServeDir::new("public").not_found_service(ServeFile::new("public/errors/404.html"));
  let serve_dir = get_service(serve_dir).handle_error(handle_error);

  Router::new()
    .nest_service("/public", serve_dir.clone())
    .fallback_service(serve_dir)
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
