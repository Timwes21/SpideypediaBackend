use mongodb::{
    Collection, bson::{Document}
};
use crate::mongo_handler::{get_collection, Users};
use crate::redis_handler::get_client;

#[derive(Clone)]
pub struct AppState {
    pub collection: Collection<Users>,
    pub redis_client: redis::Client,
}

pub async fn build_state()-> AppState{
    let collection = get_collection().await;

    let redis_client = get_client();

    AppState {collection, redis_client}

}
