use crate::axum::AxumState;
use axum::extract::ws::CloseFrame;
use axum::extract::ws::Message as AxMessage;
use axum::extract::ws::Utf8Bytes as AxUtf8Bytes;
use axum::extract::{FromRequest, Request, State, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use axum::Error as AxError;
use axum_proxy::{AppendPrefix, ReusedService};
use futures_util::StreamExt;
use axum::http::StatusCode;
use tokio::runtime::Handle;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite as ts;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tower_service::Service;

// This file requires these additional dependencies of features. They are not included right now, because we don't need them for anything else
// tokio-tungstenite
// futures-util
// axum-proxy with feautures: axum

fn into_tungstenite(message: Result<AxMessage, AxError>) -> Option<Result<tungstenite::Message, ts::Error>> {
    if let Err(x) = message {
        return None;
    }
    let message = message.unwrap();
    Some(Ok(match message {
        AxMessage::Text(text) => ts::Message::Text(ts::Utf8Bytes::from(text.as_str())),
        AxMessage::Binary(binary) => ts::Message::Binary(binary),
        AxMessage::Ping(ping) => ts::Message::Ping(ping),
        AxMessage::Pong(pong) => ts::Message::Pong(pong),
        AxMessage::Close(Some(close)) => ts::Message::Close(Some(ts::protocol::CloseFrame {
            code: ts::protocol::frame::coding::CloseCode::from(close.code),
            reason: ts::Utf8Bytes::from(close.reason.as_str()),
        })),
        AxMessage::Close(None) => ts::Message::Close(None),
    }))
}

fn from_tungstenite(message: Result<ts::Message, ts::Error>) -> Option<Result<AxMessage, AxError>> {
    if let Err(x) = message {
        return Some(Err(AxError::new(x)));
    }
    let message = message.unwrap();
    match message {
        ts::Message::Text(text) => Some(Ok(AxMessage::Text(AxUtf8Bytes::from(text.as_str())))),
        ts::Message::Binary(binary) => Some(Ok(AxMessage::Binary(binary))),
        ts::Message::Ping(ping) => Some(Ok(AxMessage::Ping(ping))),
        ts::Message::Pong(pong) => Some(Ok(AxMessage::Pong(pong))),
        ts::Message::Close(Some(close)) => Some(Ok(AxMessage::Close(Some(CloseFrame {
            code: close.code.into(),
            reason: AxUtf8Bytes::from(close.reason.as_str()),
        })))),
        ts::Message::Close(None) => Some(Ok(AxMessage::Close(None))),
        // we can ignore `Frame` frames as recommended by the tungstenite maintainers
        // https://github.com/snapview/tungstenite-rs/issues/268
        ts::Message::Frame(_) => None,
    }
}

pub async fn websocket_proxy(
    state: State<AxumState>,
    req: Request,
) -> Response {
    let is_ws = req
        .headers()
        .get("upgrade")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("websocket"))
        .unwrap_or(false);
    // println!("isWebsocket: {}, {}", is_ws, req.uri());
    if !is_ws {
        let mut proxy = axum_proxy::builder_http("localhost:5173").unwrap().build(AppendPrefix("/panel"));
        let d = ReusedService::call(&mut proxy, req).await.unwrap().unwrap();
        return d.into_response();
    }
    println!("starting websocket proxy");
    let Ok(ws) = WebSocketUpgrade::from_request(req, &()).await else {
        return (StatusCode::BAD_REQUEST, "Invalid websocket upgrade").into_response();
    };
    ws.on_upgrade(|upgraded| async move {
        let request = "http://localhost:4773/panel".into_client_request().unwrap();
        let (stream, response) = tokio_tungstenite::connect_async(request).await.unwrap();
        let (mut p_write, p_read) = stream.split();
        let (mut r_write, r_read) = upgraded.split();
        Handle::current().spawn(async move {
            if let Err(p1) = r_read
                .filter_map(|x1| async move { into_tungstenite(x1) })
                .forward(&mut p_write)
                .await {
                println!("p1 Error = {:?}", p1);
            }
        });
        if let Err(p2) = p_read
            .filter_map(|x1| async move { from_tungstenite(x1) })
            .forward(&mut r_write)
            .await {
            println!("p2 Error = {:?}", p2);
        };
    })
}
