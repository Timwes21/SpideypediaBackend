use futures_util::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use axum::{Json, extract::{Multipart, State}, http::StatusCode, response::IntoResponse};
use crate::{
    app_state::AppState, encryption, mongo_handler::Users, routes::{self, route_errors::RouteError}
};


#[derive(Deserialize, Debug)]
pub struct TokenPayload{
    token: String
}

#[axum::debug_handler]
pub async fn logout(State(state): State<AppState>, Json(payload): Json<TokenPayload>){
    let result = state.collection.update_one(doc! {"tokens": &payload.token}, doc! {"$pull": {"tokens": payload.token}}).await;
    if let Err(e) = result {
        println!("error forgetting a token: {e:?}");
    }
}

#[derive(Deserialize, Debug)]
pub struct UserInfoPayload {
    username: String,
    password: String,
    email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Token{
    token: String,
    message: String
}

impl Token{
    fn json(token: String)->Json<Self>{
        Json(Self { token, message: "Success".to_string() })
    }
}



pub async fn create_user(State(state): State<AppState>, Json(payload): Json<UserInfoPayload>)->Result<(StatusCode, Json<Token>), RouteError>{
    println!("In creating user route user");
    let filter = doc! {"userInfo.username": &payload.username};
    let mut cursor = state.collection.find(filter).await?;

    while let Some(a) = cursor.next().await{
        let user = a?;
        if payload.username == user.user_info.username{
            return Err(RouteError::UsernameExists);
        }
    }


    let new_user = Users::new(payload.username, payload.password, payload.email);


    state.collection.insert_one(&new_user).await?;
    Ok((StatusCode::CREATED, Token::json(new_user.tokens[0].to_owned())))
}


pub async fn login(State(state): State<AppState>, Json(payload): Json<UserInfoPayload>) -> Result< (StatusCode, Json<Token>), RouteError>{
    let user = state.collection.find_one(doc! {"userInfo.username": &payload.username}).await?.ok_or(RouteError::MongoError)?;
    let decrypted_password = encryption::decrypt(user.user_info.password.iv.as_str(), user.user_info.password.encrypted.as_str()).map_err(|_| RouteError::CryptError)?;
    if decrypted_password != payload.password {
        return Err(RouteError::IncorrectPassword);
    }
    let new_token = encryption::get_token();
    state.collection.update_one(doc! {"userInfo.username": payload.username}, doc! {"$push": {"tokens": &new_token}}).await.map_err(|_resp| RouteError::MongoError)?;
    Ok((StatusCode::OK, Token::json(new_token)))
        

}

