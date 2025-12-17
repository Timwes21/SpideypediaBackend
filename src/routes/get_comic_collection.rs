use axum::{
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade, Message}
    },
    response::{IntoResponse, Response},
};
use futures_util::StreamExt;
use redis::{ AsyncCommands, PubSub, RedisError, ToRedisArgs, TypedCommands};
use mongodb::{
    bson::{self, Document, doc}
};
use crate::{app_state::AppState};

// relying on unwrap until I figure out how to use 
// handle_socket in a result enum with ws.on_upgrade
// or until I find another way


pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move|socket| handle_socket(socket, state))
}


async fn handle_socket(mut socket: WebSocket, state: AppState)  {
    println!("in handle socket");
    let msg_from_client = socket.recv().await.unwrap().unwrap();
    let token = msg_from_client.to_text().unwrap();

    let user = state.collection.find_one(doc! {"tokens": &token}).await.unwrap();
    if let Some(user) = user {
        let json_user = serde_json::to_string(&user.characters).unwrap();
        if socket.send( Message::Text(json_user.into())).await.is_err() {
            println!("there was an error sending the message");
            return;
        }
    }
    let client = state.redis_client;
    let pub_sub = client.get_async_pubsub().await.unwrap();
    let (mut sink, mut stream) = pub_sub.split();
    sink.subscribe("charUpdates").await.unwrap();
    while let Some(msg) = stream.next().await {
        let payload: String = msg.get_payload().unwrap();
        if payload == token {
            let user = state.collection.find_one(doc! {"tokens": &token}).await.unwrap();
            if let Some(user) = user {
                let json_users = serde_json::to_string(&user.characters).unwrap();
                if socket.send( Message::Text(json_users.into())).await.is_err() {
                    println!("there was an error sending the message");
                    return;
                }
            }
            

        }
    }

}