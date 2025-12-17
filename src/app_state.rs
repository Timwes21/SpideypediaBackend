use mongodb::{
    Collection, bson::{Document}
};
use crate::mongo_handler::{get_collection, Users};
use crate::redis_handler::get_client;
use crate::minio_client::MinioClient;

#[derive(Clone)]
pub struct AppState {
    pub collection: Collection<Users>,
    pub redis_client: redis::Client,
    pub minio_client: MinioClient
}

pub async fn build_state()-> AppState{
    let collection = get_collection().await;
    let minio_client = MinioClient::new().await;

    let redis_client = get_client();

    AppState {collection, redis_client, minio_client}

}
