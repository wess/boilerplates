//
// provider.ts
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use async_trait::async_trait;
use chrono::Utc;

use mongodb::{
  error::Error,
  Collection,
  results::{
    InsertOneResult,
    UpdateResult,
    DeleteResult
  },
  bson::{
    Document,
    doc,
  }
};

use crate::data::connection::Connection;
use crate::data::models::model::Model;

#[async_trait]
pub trait Provider<T:Model + std::marker::Sync + std::marker::Send> {

  fn init(conn:&Connection) -> Self;
  fn collection(&self) -> &Collection;

  async fn insert<'a>(&self, model: &'a T) -> Result<InsertOneResult, Error> {
    let mut document = model.to_bson().as_document_mut().unwrap().clone();

    document.insert("created_at", Utc::now());
    document.insert("updated_at", Utc::now());

    self.collection().insert_one(
      document,
      None
    ).await
  }

  async fn update<'a>(&self, model: &'a T)  -> Result<UpdateResult, Error> {
    let id = model.id().unwrap();
    let filter = doc! {"_id": id};
    let document = model.to_bson().as_document().unwrap().clone();
    let mut update = Document::new();

    update.insert("$set", document);
    update.insert("updated_at", Utc::now());

    self.collection().update_one(
      filter, 
      update,
      None
    ).await
  }

  async fn delete<'a>(&self, model: &'a T) -> Result<DeleteResult, Error> {
    let id = model.id().unwrap();
    let filter = doc! {"_id": id};

    
    self.collection().delete_one(
      filter, 
      None
    ).await
  }
}
