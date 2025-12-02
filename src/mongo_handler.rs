use mongodb::{
    Client, Collection, bson::{Document, oid::ObjectId, doc}
};



use serde::{Deserialize, Serialize};

use crate::encryption::{encrypt, get_token};

#[derive(Deserialize, Serialize, Debug)]
pub struct Password {
    pub iv: String,
    pub encrypted: String
}

impl Password{
    fn new(password: String)->Self{
        let (iv, encrypted) = encrypt(&password).unwrap();
        Self{iv, encrypted}

    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: Password,
    pub email: String,
}

impl UserInfo {
    fn new(username: String, password: String, email: Option<String>)->Self{
        let password = Password::new(password);
        let email = match email {
            Some(e)=> e,
            None => "".to_string()
        };
        
        Self{ username, password, email }
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Users{
    pub tokens: Vec<String>,
    #[serde(rename="userInfo")]
    pub user_info: UserInfo,
    pub characters: Document
}


impl Users {
    pub fn new(username: String, password: String, email: Option<String>)-> Self{
        let user_info = UserInfo::new(username, password, email);
        let token = get_token();
        let mut tokens: Vec<String> = Vec::new();
        tokens.push(token);
        Self { tokens, user_info, characters: doc!{}}
    }
}



pub async fn get_collection() -> Collection<Users>{
    let db_connection_str = std::env::var("MONGO_URL").unwrap();
    let client = Client::with_uri_str(db_connection_str).await.unwrap();

    let db = client.database("comicManagement");

    let collection: Collection<Users> = db.collection("users");
    collection
}
