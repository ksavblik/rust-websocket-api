use actix_web::{web, get, post, patch, delete, HttpResponse, Responder};
use entity::{prelude::Book, book::ActiveModel};
use migration::sea_orm::{EntityTrait, ActiveValue, ActiveModelTrait};

use crate::AppState;
use crate::request_payloads::{CreateBook, UpdateBook, ToActiveModel};

pub fn book_service(cfg: &mut web::ServiceConfig) {
  cfg.service(get_books);
  cfg.service(get_book);
  cfg.service(create_book);
  cfg.service(patch_book);
  cfg.service(delete_book);
}

#[get("/books")]
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db_conn;
    let all_books = Book::find().all(db).await.unwrap();
    let json = serde_json::to_string(&all_books).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[get("/book/{id}")]
async fn get_book(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let db = &data.db_conn;
    let result = Book::find_by_id(id.into_inner()).one(db).await.unwrap();
    if let Some(book) = result {
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&book).unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[post("/book")]
async fn create_book(data: web::Data<AppState>, info: web::Json<CreateBook>) -> impl Responder {
    let db = &data.db_conn;
    let active_model = info.into_inner().to_active_model();
    let inserted_model = active_model.insert(db).await.unwrap();
    HttpResponse::Created().content_type("application/json").body(serde_json::to_string(&inserted_model).unwrap())
}

#[patch("/book/{id}")]
async fn patch_book(data: web::Data<AppState>, id: web::Path<i32>, info: web::Json<UpdateBook>) -> impl Responder {
    let db = &data.db_conn;
    let mut active_model: ActiveModel = info.into_inner().to_active_model();
    active_model.id = ActiveValue::Set(id.into_inner());
    let updated_book = active_model.update(db).await.unwrap();
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&updated_book).unwrap())
}

#[delete("/book/{id}")]
async fn delete_book(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
  let db = &data.db_conn;
  let res = Book::delete_by_id(id.into_inner()).exec(db).await.unwrap();
  if res.rows_affected != 1 {
    return HttpResponse::NotFound().finish();
  }
  HttpResponse::Ok().finish()
}