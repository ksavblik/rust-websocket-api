use chrono::Utc;
use entity::book::ActiveModel;
use sea_orm::{prelude::DateTimeUtc, ActiveValue, IntoActiveValue};
use serde::{Deserialize, Serialize};

pub trait ToActiveModel<T> {
    fn to_active_model(self) -> T;
}

fn to_active_value<T>(opt: Option<T>) -> ActiveValue<T>
where
    sea_orm::Value: std::convert::From<T>,
{
    if let Some(v) = opt {
        return ActiveValue::Set(v);
    }
    ActiveValue::NotSet
}

fn opt_to_active_value<T: migration::Nullable>(opt: Option<T>) -> ActiveValue<Option<T>>
where
    sea_orm::Value: std::convert::From<T>,
{
    if opt.is_some() {
        return ActiveValue::Set(opt);
    }
    ActiveValue::NotSet
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub price: Option<String>,
    pub publish_date: Option<DateTimeUtc>,
}

impl ToActiveModel<ActiveModel> for CreateBook {
    fn to_active_model(self) -> ActiveModel {
        let timestamp = Utc::now();
        ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::Set(self.title),
            author: ActiveValue::Set(self.author),
            price: self.price.into_active_value(),
            publish_date: self.publish_date.into_active_value(),
            created_at: ActiveValue::Set(timestamp),
            updated_at: ActiveValue::Set(timestamp),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub price: Option<String>,
    pub publish_date: Option<DateTimeUtc>,
    pub updated_at: DateTimeUtc,
}

impl ToActiveModel<ActiveModel> for UpdateBook {
    fn to_active_model(self) -> ActiveModel {
        let timestamp = Utc::now();
        ActiveModel {
            id: ActiveValue::NotSet,
            title: to_active_value(self.title),
            author: to_active_value(self.author),
            price: opt_to_active_value(self.price),
            publish_date: opt_to_active_value(self.publish_date),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::Set(timestamp),
        }
    }
}
