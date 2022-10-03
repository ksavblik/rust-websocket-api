use actix_web::{middleware, web, App, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

mod request_payloads;

mod book_routes;
use book_routes::book_service;

mod errors;

use entity::book::Model;

#[derive(Debug, Clone)]
struct AppState {
    db_conn: DatabaseConnection,
    broadcaster: UnboundedSender<DbAction>,
    // ws_clients: Arc<Mutex<Vec<ws::WebsocketContext<>>>>
}

enum DbAction {
    Created(Model),
    Updated(Model),
    Deleted(i32),
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");
    let connection = sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed connection to the database");

    Migrator::up(&connection, None)
        .await
        .expect("Failed to run the migration");

    let (tx, mut rx): (UnboundedSender<DbAction>, UnboundedReceiver<DbAction>) =
        unbounded_channel();

    let state = AppState {
        db_conn: connection,
        broadcaster: tx,
    };

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            // println!("GOT = {}", message);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(book_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
