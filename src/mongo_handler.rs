use mongodb::{
    Client, Collection, bson::{Document, oid::ObjectId}
};


use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Password {
    pub iv: String,
    pub encrypted: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub username: String,
    pub password: Password,
    pub email: String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct Users{
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub tokens: Vec<String>,
    #[serde(rename="userInfo")]
    pub user_info: UserInfo,
    pub characters: Document
    
    

}



pub async fn get_collection() -> Collection<Users>{
    let db_connection_str = std::env::var("MONGO_URL").unwrap();
    let client = Client::with_uri_str(db_connection_str).await.unwrap();

    let db = client.database("comicManagement");

    let collection: Collection<Users> = db.collection("users");
    collection
}
