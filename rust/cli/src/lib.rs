#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;

mod result;
pub use result::Result;

#[macro_use]
extern crate oxide;

pub async fn run() -> Result<()> {
  Ok(())
}