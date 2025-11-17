use axum::{Json, extract::{Multipart, State}, http::StatusCode};
use redis::{ AsyncCommands, Client, PubSub, RedisError, ToRedisArgs, TypedCommands};
use serde::Serialize;
use std::any::type_name_of_val;
use mongodb::{
    bson::{self, Document, doc}, error::Error, results::UpdateResult
};
use std::fmt::Display;
use crate::{app_state::AppState, mongo_handler::Users, routes};
use routes::json_responses::{CharacterLoad, DeleteIssueData, AddToCharacterData, CharacterData};


async fn publish(redis_client: Client, token: String){
    let mut con = redis_client.get_connection().unwrap();
    if let Err(e) = con.publish("charUpdates", token) {
        println!("there was an error publishing: {e:?}");
    }
}


pub async fn add_character(State(state): State<AppState>, Json(payload): Json<CharacterLoad>){
    println!("add character route");
    println!("{payload:?}");
    let path = format!("characters.{}", payload.character);




    let filter = payload.get_filter();
    let update = doc! {"$set":{path: {}}};
    let result = state.collection.update_one(filter, update).await;

    if let Err(e) = result {
        println!("There was an error adding a character: {e:?}")
    }
    publish(state.redis_client, payload.token).await;

}

pub async fn add_title(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>){
    let filter = payload.get_filter();
    let update = payload.get_update("$set");
    let result = state.collection.update_one(filter, update).await;
    if let Err(e) = result {
        println!("There was an error adding a title: {e:?}");
    }
    publish(state.redis_client, payload.token).await;
}





pub async fn delete_issue(State(state): State<AppState>, Json(payload): Json<DeleteIssueData>){
    let filter = payload.get_filter();
    let update = payload.get_update();
    let result = state.collection.update_one(filter, update).await;
    if let Err(e) = result {
        println!("there was an error deleting an issue: {e:?}");
    }
    publish(state.redis_client, payload.token).await;
    
        
        
}

fn bad_req<E: Display>(e: E)-> (StatusCode, String){
    (StatusCode::BAD_REQUEST, e.to_string())

}

fn int_er<E: Display>(e: E)-> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

#[axum::debug_handler]
pub async fn update_details(State(state): State<AppState>, mut multipart: Multipart){
    println!("here in update details");
    let mut token: Option<String> = None;
    let mut issue_details: Option<Document> = None;
    let mut character_data: Option<CharacterData> = None;
    
    while let Some(field) = multipart.next_field().await.map_err(int_er).unwrap() {
        let name = field.name().unwrap_or("");
        println!("{}", name);
        
        match name {
            "token" => {
                token = Some(field.text().await.map_err(bad_req).unwrap())
            },
            "characterData" => {
                let s = field.text().await.map_err(bad_req).unwrap();
                character_data = Some(serde_json::from_str(s.as_str()).unwrap());
            },
            "issueDetailList" => {
                let s = field.text().await.map_err(bad_req).unwrap();
                issue_details = Some(serde_json::from_str(s.as_str()).unwrap());
            },
            _=> {}
        }
        
        
    }
    
    let character_data = character_data.unwrap();
    
    let filter = doc! {"tokens": token.as_ref().unwrap()};
    let path = format!("{}.issueRundown", character_data.get_path());
    let update = doc! {"$set": {path: issue_details.unwrap()}};

    let result = state.collection.update_one(filter, update).await;
    
    if let Err(e) = result {
        println!("there was an error updating details: {e:?}");

    }
    publish(state.redis_client, token.unwrap()).await;


}





pub async fn delete_character( State(state): State<AppState>, Json(payload): Json<CharacterLoad>){
    print!("in delete character");
    let filter = payload.get_filter();
    let path = format!("characters.{}", payload.character);

    let result = state.collection.update_one(filter, doc! {"$unset": {path: ""}}).await;    

    if let Err(e) = result {
        println!("there was an error deleting a character: {e:?}");
    }
    publish(state.redis_client, payload.token).await;
}

pub async fn add_volume(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>){
    println!("in add volume");
    let filter = payload.get_filter();
    let update = payload.get_update("$set");

    let result = state.collection.update_one(filter, update).await;

    if let Err(e) = result {
        println!("There was an error adding a volume: {e:?}");
    }
    publish(state.redis_client, payload.token).await;

}

pub async fn delete_volume(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>){
    println!("in add volume");
    let filter = payload.get_filter();
    let update = payload.get_update("$unset");

    let result = state.collection.update_one(filter, update).await;

    if let Err(e) = result {
        println!("There was an error adding a volume: {e:?}");
    }
    publish(state.redis_client, payload.token).await;
}
