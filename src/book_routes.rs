use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use entity::{book::ActiveModel, prelude::Book};
use migration::sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use crate::errors::ApiError;
use crate::request_payloads::{CreateBook, ToActiveModel, UpdateBook};
use crate::AppState;

pub fn book_service(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books);
    cfg.service(get_book);
    cfg.service(create_book);
    cfg.service(patch_book);
    cfg.service(delete_book);
}

#[get("/books")]
async fn get_books(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let db = &data.db_conn;
    let all_books = Book::find().all(db).await?;
    let json = serde_json::to_string(&all_books)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

#[get("/book/{id}")]
async fn get_book(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let db = &data.db_conn;
    let result = Book::find_by_id(id.into_inner()).one(db).await?;
    if let Some(book) = result {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&book)?));
    }
    Err(ApiError::NotFoundError)
}

#[post("/book")]
async fn create_book(
    data: web::Data<AppState>,
    info: web::Json<CreateBook>,
) -> Result<impl Responder, ApiError> {
    let db = &data.db_conn;
    let active_model = info.into_inner().to_active_model();
    let inserted_model = active_model.insert(db).await?;
    Ok(HttpResponse::Created()
        .content_type("application/json")
        .body(serde_json::to_string(&inserted_model)?))
}

#[patch("/book/{id}")]
async fn patch_book(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    info: web::Json<UpdateBook>,
) -> Result<impl Responder, ApiError> {
    let db = &data.db_conn;
    let mut active_model: ActiveModel = info.into_inner().to_active_model();
    active_model.id = ActiveValue::Set(id.into_inner());
    let updated_book = active_model.update(db).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&updated_book)?))
}

#[delete("/book/{id}")]
async fn delete_book(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let db = &data.db_conn;
    let res = Book::delete_by_id(id.into_inner()).exec(db).await?;
    if res.rows_affected != 1 {
        return Err(ApiError::NotFoundError);
    }
    Ok(HttpResponse::Ok().finish())
}
