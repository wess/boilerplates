//
// model.rs
// Spose
// 
// Author: Wess (me@wess.io)
// Created: 12/08/2020
// 
// Copywrite (c) 2020 Wess.io
//

use serde::{Serialize};

use mongodb::bson::{
  to_bson,
  Bson,
  oid::ObjectId
};

pub trait Model : Serialize {
  fn id(&self) -> Option<ObjectId> {
    let b = self.to_bson();
    let d = b.as_document().unwrap();

    Some(
      d.get("_id")
      .unwrap()
      .as_object_id()
      .unwrap()
      .clone()
    )
  }

  fn to_bson(&self) -> Bson {
    to_bson(self).unwrap()
  }
}
