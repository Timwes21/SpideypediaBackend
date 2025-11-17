use mongodb::error::Error;
use redis::{Client, Commands, RedisError};
use futures_util;



pub fn get_client() ->redis::Client{
    let url = std::env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url).unwrap();
    client
}
