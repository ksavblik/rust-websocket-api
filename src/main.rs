use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post, patch, delete};
use dotenv::dotenv;
use sea_orm::{
  DatabaseConnection,
  EntityTrait,
};

use migration::{Migrator, MigratorTrait, sea_orm::{ActiveModelTrait, ActiveValue}};
use entity::{prelude::*, post::ActiveModel};

pub mod post;
use post::{CreatePost, PatchPost};

#[derive(Debug, Clone)]
struct AppState {
  conn: DatabaseConnection,
}

#[get("/posts")]
async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;
    let all_posts = Post::find().all(conn).await.unwrap();
    let json = serde_json::to_string(&all_posts).unwrap();
    HttpResponse::Ok().content_type("application/json").body(json)
}

#[get("/post/{id}")]
async fn get_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let conn = &data.conn;
    let result = Post::find_by_id(id.into_inner()).one(conn).await.unwrap();
    if let Some(post) = result {
        return HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&post).unwrap());
    }
    HttpResponse::NotFound().finish()
}

#[post("/post")]
async fn create_post(data: web::Data<AppState>, info: web::Json<CreatePost>) -> impl Responder {
    let conn = &data.conn;
    let active_model = info.into_inner().to_active_model();
    let inserted_model = active_model.insert(conn).await.unwrap();
    HttpResponse::Created().content_type("application/json").body(serde_json::to_string(&inserted_model).unwrap())
}

#[patch("/post/{id}")]
async fn patch_post(data: web::Data<AppState>, id: web::Path<i32>, info: web::Json<PatchPost>) -> impl Responder {
    let conn = &data.conn;
    let mut active_model: ActiveModel = info.into_inner().to_active_model();
    active_model.id = ActiveValue::Set(id.into_inner());
    let updated_post = active_model.update(conn).await.unwrap();
    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&updated_post).unwrap())
}

#[delete("/post/{id}")]
async fn delete_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
  let conn = &data.conn;
  let res = Post::delete_by_id(id.into_inner()).exec(conn).await.unwrap();
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

    let state = AppState { conn: connection };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(get_posts)
            .service(get_post)
            .service(create_post)
            .service(patch_post)
            .service(delete_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}