use entity::post::ActiveModel;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

pub fn option_into_active_value<T>(option: Option<T>) -> ActiveValue<T> where sea_orm::Value: std::convert::From<T> {
  if let Some(v) = option {
    return ActiveValue::Set(v);
  }
  ActiveValue::NotSet
}

#[derive(Deserialize, Serialize)]
pub struct PatchPost {
    pub title: Option<String>,
    pub text: Option<String>,
}

impl PatchPost {
  pub fn to_active_model(self) -> ActiveModel {
    ActiveModel { id: ActiveValue::NotSet, title: option_into_active_value(self.title), text: option_into_active_value(self.text) }
  }
}

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub title: String,
    pub text: String,
}

impl CreatePost {
  pub fn to_active_model(self) -> ActiveModel {
    ActiveModel { id: ActiveValue::NotSet, title: ActiveValue::Set(self.title), text: ActiveValue::Set(self.text) }
  }
}