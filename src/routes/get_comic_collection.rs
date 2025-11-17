use axum::{
    Json,
    body::Body,
    http::StatusCode,
    extract::{
        Multipart, State, Path,
        ws::{WebSocket, WebSocketUpgrade, Message}
    },
    response::{IntoResponse, Response},
};
use futures_util::StreamExt;
use redis::{ AsyncCommands, PubSub, RedisError, ToRedisArgs, TypedCommands};
use serde::{Deserialize, Serialize};
use std::{any::type_name_of_val, fmt::Error};
use mongodb::{
    Collection, bson::{self, Document, doc}, results::UpdateResult
};
use crate::{app_state::AppState, mongo_handler::Users, routes};




pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    println!("made it");
    ws.on_upgrade(move|socket| handle_socket(socket, state))
}


async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let msg_from_client = socket.recv().await.unwrap().unwrap();
    let token = msg_from_client.to_text().unwrap();

    let user = state.collection.find_one(doc! {"tokens": &token}).await.unwrap();
    if let Some(user) = user {
        println!("found user");
        let json_user = serde_json::to_string(&user).unwrap();
        if socket.send( Message::Text(json_user.into())).await.is_err() {
            println!("there was an error");
            return;
        }
    }
    let client = state.redis_client;
    let pub_sub = client.get_async_pubsub().await.unwrap();
    let (mut sink, mut stream) = pub_sub.split();
    sink.subscribe("charUpdates").await.unwrap();
    while let Some(msg) = stream.next().await{
        let payload: String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);
        if payload == token {
            let user = state.collection.find_one(doc! {"tokens": &token}).await.unwrap();
            if let Some(user) = user {
                let json_users = serde_json::to_string(&user).unwrap();
                if socket.send( Message::Text(json_users.into())).await.is_err() {
                    // client disconnected
                    return;
                }
            }
            

        }
    }
}