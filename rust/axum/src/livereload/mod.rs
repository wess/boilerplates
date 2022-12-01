mod inject;
mod poll;
mod overlay;
mod predicate;
mod polyfill;

use http::{header, Request, Response, StatusCode};
use inject::InjectService;
use poll::Polling;
use overlay::OverlayService;
use predicate::ContentTypeStartsWithPredicate;
use tokio::sync::broadcast::Sender;
use tower::{Layer, Service};

use crate::views::TEMPLATES;

#[derive(Clone, Debug)]
pub struct Reloader {
  sender: Sender<()>,
}

impl Reloader {
  pub fn new() -> Self {
    let (sender, _) = tokio::sync::broadcast::channel(1);
    
    Self { sender }
  }

  pub fn reload(&self) {
    self.sender.send(()).ok();
  }
}

#[derive(Clone, Debug)]
pub struct LiveReloadLayer {
  custom_prefix: Option<String>,
  reloader: Reloader,
}


impl LiveReloadLayer {
  pub fn new() -> LiveReloadLayer {
    LiveReloadLayer {
        custom_prefix: None,
        reloader: Reloader::new(),
    }
  }

  pub fn with_custom_prefix<P: Into<String>>(prefix: P) -> LiveReloadLayer {
    LiveReloadLayer {
      custom_prefix: Some(prefix.into()),
      reloader: Reloader::new(),
    }
  }

  pub fn reloader(&self) -> Reloader {
    self.reloader.clone()
  }
}


impl<S> Layer<S> for LiveReloadLayer {
  type Service = LiveReload<S>;

  fn layer(&self, inner: S) -> Self::Service {
    if let Some(ref custom_prefix) = self.custom_prefix {
      LiveReload::with_custom_prefix(inner, self.reloader.clone(), custom_prefix.clone())
    } else {
      LiveReload::new(inner, self.reloader.clone())
    }
  }
}


type InnerService<S> = OverlayService<
  String,
  http::Error,
  OverlayService<
    Polling,
    http::Error,
    InjectService<S, ContentTypeStartsWithPredicate<&'static str>>,
  >,
>;

#[derive(Clone, Debug)]
pub struct LiveReload<S> {
    service: InnerService<S>,
}

impl<S> LiveReload<S> {
  pub fn new(service: S, reloader: Reloader) -> Self {
    Self::with_custom_prefix(
      service,
      reloader,
      "/axum-tower-reload/_________no_collisions",
    )
  }

  pub fn with_custom_prefix<P: Into<String>>(service: S, reloader: Reloader, prefix: P) -> Self {
    let prefix = prefix.into();
    let long_poll_path = format!("{}/long-poll", prefix);
    let back_up_path = format!("{}/back-up", prefix);

    let mut inject_context = tera::Context::new();
    inject_context.insert("long_poll", &long_poll_path);
    inject_context.insert("back_up", &back_up_path);

    let html = TEMPLATES.render("_livereload.html", &inject_context).unwrap();

    let inject = InjectService::new(
      service,
      html.into(),
      ContentTypeStartsWithPredicate::new("text/html"),
    );

    let overlay_poll = OverlayService::new(inject).path(long_poll_path, move || {
      Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .body(Polling::new(reloader.sender.subscribe()))
    });

    let overlay_up = OverlayService::new(overlay_poll).path(back_up_path, || {
      Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body("Ok".to_owned())
    });

    LiveReload {
        service: overlay_up,
    }
  }
}


impl<ReqBody, RespBody, S> Service<Request<ReqBody>> for LiveReload<S>
where
  S: Service<Request<ReqBody>, Response = Response<RespBody>>,
  RespBody: http_body::Body,
{
  type Response = <InnerService<S> as Service<Request<ReqBody>>>::Response;
  type Error = <InnerService<S> as Service<Request<ReqBody>>>::Error;
  type Future = <InnerService<S> as Service<Request<ReqBody>>>::Future;


  fn poll_ready(
    &mut self,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
    self.service.call(req)
  }
}
