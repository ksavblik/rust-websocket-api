use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post, patch, delete};
use dotenv::dotenv;
use sea_orm::{
  DatabaseConnection,
  EntityTrait,
};

use migration::{Migrator, MigratorTrait, sea_orm::{ActiveModelTrait, ActiveValue}};
use entity::{prelude::*, book::ActiveModel};

pub mod request_payloads;
use request_payloads::{CreateBook, UpdateBook};

use crate::request_payloads::ToActiveModel;

#[derive(Debug, Clone)]
struct AppState {
  db_conn: DatabaseConnection,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");
    let connection = sea_orm::Database::connect(&database_url).await.expect("Failed connection to database");
    Migrator::up(&connection, None).await.unwrap();

    let state = AppState { db_conn: connection };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(get_books)
            .service(get_book)
            .service(create_book)
            .service(patch_book)
            .service(delete_book)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}