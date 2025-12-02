use axum::{Json, extract::{Multipart, State}, response::IntoResponse};
use redis::{ AsyncCommands, Client, PubSub, RedisError, ToRedisArgs, TypedCommands};
use mongodb::bson::{Document, doc};
use bytes::Bytes;
use crate::{app_state::AppState, routes, routes::route_errors::RouteError};
use routes::json_responses::{CharacterLoad, DeleteIssueData, AddToCharacterData, CharacterData};


async fn publish(redis_client: Client, token: String)->Result<(), redis::RedisError>{
    let mut con = redis_client.get_connection()?;
    if let Err(e) = con.publish("charUpdates", token) {
        println!("there was an error publishing: {e:?}");
    }
    Ok(())
}


pub async fn add_character(State(state): State<AppState>, Json(payload): Json<CharacterLoad>)->Result<(), RouteError>{
    println!("add character route");
    println!("{payload:?}");
    let path = format!("characters.{}", payload.character);




    let filter = payload.get_filter();
    let update = doc! {"$set":{path: {}}};
    state.collection.update_one(filter, update).await?;


    publish(state.redis_client, payload.token).await?;
    Ok(())

}

pub async fn add_title(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>)->Result<(), RouteError>{
    let filter = payload.get_filter();
    let update = payload.get_update();
    state.collection.update_one(filter, update).await?;
    publish(state.redis_client, payload.token).await?;
    Ok(())
}





pub async fn delete_issue(State(state): State<AppState>, Json(payload): Json<DeleteIssueData>)->Result<(), RouteError>{
    let filter = payload.get_filter();
    let update = payload.get_update();
    state.collection.update_one(filter, update).await?;
    
    publish(state.redis_client, payload.token).await?;
    Ok(())
    
        
        
}


#[axum::debug_handler]
pub async fn update_details(State(state): State<AppState>, mut multipart: Multipart)->Result<impl IntoResponse, RouteError>{
    println!("here in update details");
    let mut token: Option<String> = None;
    let mut issue_details: Option<Document> = None;
    let mut character_data: Option<CharacterData> = None;
    let mut image_bytes:Option<Bytes> = None;
    let mut previous_image_name: Option<String> = None;
    
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("");
        
        match name {
            "token" => {
                token = field.text().await.ok();
            },
            "characterData" => {
                let s = field.text().await?;
                character_data = serde_json::from_str(s.as_str()).ok();
            },
            "issueDetailList" => {
                let s = field.text().await?;
                issue_details = serde_json::from_str(s.as_str()).ok();
            },
            "image" => {
                image_bytes = field.bytes().await.ok();
            },
            "prevImageName" => {
                let s = field.text().await?;
                previous_image_name = serde_json::from_str(s.as_str()).ok();
            }
            _ => {}
        }
    }
    
    let character_data = character_data.ok_or(RouteError::OptionError)?;
    let token = token.as_ref().ok_or(RouteError::OptionError)?;
    let issue_details = issue_details.ok_or(RouteError::OptionError)?;

    let filter = doc! {"tokens": token};
    let path = format!("{}.issueRundown", character_data.get_path());
    let update = doc! {"$set": {path: issue_details}};

    state.collection.update_one(filter, update).await?;

    if let Some(bytes) = image_bytes{
        let user = state.collection.find_one(doc! {"tokens": token}).await?.ok_or(RouteError::MongoError)?;
        let key = state.minio_client.add_object(bytes, &user.user_info.username).await?;
        let path = format!("{}.imageName", character_data.get_path());
        state.collection.update_one(doc! {"tokens": token}, doc! {"$set": {path: key}}).await?;
        if let Some(previous_image_name) = previous_image_name{
            state.minio_client.delete_object(previous_image_name, &user.user_info.username).await?;
        }


        

    }
    
    publish(state.redis_client, token.to_owned()).await?;
    Ok(())


}





pub async fn delete_character( State(state): State<AppState>, Json(payload): Json<CharacterLoad>)->Result<(), RouteError>{
    print!("in delete character");
    let filter = payload.get_filter();
    let path = format!("characters.{}", payload.character);

    state.collection.update_one(filter, doc! {"$unset": {path: ""}}).await?;    

    publish(state.redis_client, payload.token).await?;
    Ok(())
}

pub async fn add_volume(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>)->Result<(), RouteError>{
    println!("in add volume");
    let filter = payload.get_filter();
    let update = payload.get_update();

    state.collection.update_one(filter, update).await?;

    publish(state.redis_client, payload.token).await?;
    Ok(())

}

pub async fn delete_volume(State(state): State<AppState>, Json(payload): Json<AddToCharacterData>)->Result<(), RouteError>{
    println!("in delete volume");
    let filter = payload.get_filter();
    let update = payload.get_remove();
    

    state.collection.update_one(filter, update).await?;

    if let Some(image_names) = payload.image_names{
        let user = state.collection.find_one(doc! {"tokens": &payload.token}).await?.ok_or(RouteError::OptionError)?;
        for image_name in image_names{
            state.minio_client.delete_object(image_name, &user.user_info.username).await?;

        }
    }


    
    publish(state.redis_client, payload.token).await?;
    Ok(())
}
