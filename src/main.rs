use std::{collections::HashMap, sync::Arc};

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use uuid::Uuid;

use log::debug;

mod book_routes;
mod errors;
mod request_payloads;
mod websocket;

use book_routes::book_service;
use entity::book::Model;
use websocket::ws_handler;

pub type WebSocketSession = Arc<Mutex<actix_ws::Session>>;

pub type WebSocketSessions = Arc<Mutex<HashMap<Uuid, WebSocketSession>>>;

#[derive(Clone)]
struct AppState {
    db_conn: DatabaseConnection,
    broadcaster: UnboundedSender<DbAction>,
    ws_clients: WebSocketSessions,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DbAction {
    Created(Model),
    Updated(Model),
    Deleted(i32),
}

impl std::fmt::Display for DbAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbAction::Created(new_book) => {
                write!(f, "Created book [id: {}]", new_book.id)
            }
            DbAction::Updated(updated_book) => {
                write!(f, "Updated book [id: {}]", updated_book.id)
            }
            DbAction::Deleted(id) => {
                write!(f, "Deleted book [id: {}]", id)
            }
        }
    }
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

    let ws_clients = Arc::new(Mutex::new(HashMap::new()));

    let state = AppState {
        db_conn: connection,
        broadcaster: tx,
        ws_clients: ws_clients.clone(),
    };

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let json = serde_json::to_string(&message).unwrap();
            debug!("Sending {} action update to WebSocket clients", message);
            for (_, ws_session) in ws_clients.lock().await.iter() {
                let mut ws_session = ws_session.lock().await;
                let _ = ws_session.text(&json).await;
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(book_service)
            .service(ws_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
