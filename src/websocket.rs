use std::sync::Arc;

use actix_web::{get, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::Message;
use log::debug;
use tokio::sync::Mutex;

use crate::{AppState, WebSocketSessions};

#[get("/ws")]
async fn ws_handler(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    debug!(
        "Accepting incoming WebSocket connection {:?}",
        req.headers().get("user-agent")
    );

    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    rt::spawn(start_ws(session, msg_stream, data.ws_clients.clone()));

    Ok(res)
}

pub async fn start_ws(
    ws_session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    ws_clients: WebSocketSessions,
) {
    let session = Arc::new(Mutex::new(ws_session));

    let uuid = uuid::Uuid::new_v4();

    ws_clients.lock().await.insert(uuid, session.clone());

    debug!(
        "Current amount of the WebSocket connections {}",
        ws_clients.lock().await.len()
    );

    let close_reason = loop {
        match msg_stream.recv().await {
            Some(Ok(msg)) => {
                match msg {
                    Message::Close(reason) => {
                        break reason;
                    }
                    Message::Ping(bytes) => {
                        let _ = session.lock().await.pong(&bytes).await;
                    }
                    _ => (),
                };
            }
            _ => break None,
        }
    };

    debug!("Closing WebSocket connection");

    let locked_session = session.lock().await.to_owned();
    let _ = locked_session.close(close_reason).await;
    let mut sessions = ws_clients.lock().await;
    sessions.remove(&uuid);
}
