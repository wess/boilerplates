use boilerplate::{
  launch,
  SystemResult
};

#[tokio::main]
async fn main() -> SystemResult<()> {
  launch().await
}
