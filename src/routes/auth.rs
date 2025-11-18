use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use axum::{Json, extract::{Multipart, State}, http::StatusCode};
use crate::{app_state::AppState, mongo_handler::Users, routes};
async fn login(){


}


async fn logout(){

}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    password: String,
    email: String,
    #[serde(rename="phoneNumber")]
    phone_number: String
}

pub async fn create_user(State(state): State<AppState>, Json(payload): Json<UserInfo>){
    println!("In creating user route user");
    let filter = doc! {"userInfo.username": payload.username};
    let cursor = state.collection.find(filter).await.unwrap();
    
        
    // try{
    //     const token = await createUser(data, collection);
    //     res.status(200).json({token: token});
    // }
    // catch(err){
    //     res.status(500).send({err: err});
    //     console.log(err);
    // }
}

