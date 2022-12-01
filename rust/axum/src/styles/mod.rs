use std::sync::{
  Arc, 
  RwLock
};

use axum::{
  Router,
  routing::get, 
  extract::{
    Path,
    State,
  },
  response::IntoResponse,
};

struct StyleState {
  scss:String
}

impl StyleState {
  fn new(path:&str) -> Self {
    Self {
      scss: string!(path)
    }
  }
}

type SharedState = Arc<RwLock<StyleState>>;

pub async fn router(path:Option<String>) -> Router {
  let path = match path {
    Some(path) => path,
    None => path.unwrap_or(format!("{}/public/styles", std::env!("CARGO_MANIFEST_DIR")))
  };

  let state = StyleState::new(&path);
  let state = Arc::new(RwLock::new(state));


  let router = 
    Router::new()
      .route("/:name", get(handler))
      .with_state(Arc::clone(&state));

  Router::new()
    .nest_service("/", router)
}

async fn handler(Path(name):Path<String>, State(state): State<SharedState>) -> impl IntoResponse {
  let components = name.split(".").into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
  
  let name:String = match components.last().unwrap().as_str() {
    "scss" => name.clone(),
    "sass" => name.clone(),
    "css" => name.clone().replace(".css", ".scss"),
    _ => panic!("Invalid file type")
  };
  
  let scss = state.read().unwrap().scss.clone();
  let path = format!("{}/{}", scss, name);

  axum::response::Response::builder()
    .header("Content-Type", "text/css")
    .body(grass::from_path(&path, &grass::Options::default()).unwrap())
    .unwrap()
}