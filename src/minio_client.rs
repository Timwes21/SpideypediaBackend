use bytes::Bytes;
use minio::s3::{
    Client, ClientBuilder, creds::StaticProvider, http::BaseUrl, segmented_bytes::SegmentedBytes, types::S3Api, builders::ObjectToDelete
};
use tokio::task::JoinError;
use std::{env, str::FromStr};

use crate::encryption;
use std::sync::Arc;


#[derive(Clone)]
pub struct MinioClient{
    pub client: Arc<Client>,
    pub bucket_name: String
}

impl MinioClient{
    pub fn new()-> Self{
        let endpoint = env::var("MINIO_URL").unwrap();
        let key = env::var("MINIO_ACCESS_KEY").unwrap();
        // let bucket_name = env::var("BUCKET_NAME");
        
        let provider = StaticProvider::new(key.as_str(), key.as_str(), None);
        
        let base_url = BaseUrl::from_str(endpoint.as_str()).unwrap();
        
        Self {
            client: Arc::new(ClientBuilder::new(base_url)
                .provider(Some(Box::new(provider)))
                .build()
                .expect("Did not build minio client")),
            bucket_name: "python-test-bucket".to_string()
        }

    }

    pub async fn add_object(&self, bytes: Bytes, username: &String)->Result<String, minio::s3::error::Error>{
        let object_name = encryption::get_token();
        let key = self.get_key(username, object_name);
        let data = SegmentedBytes::from(bytes);
        self.client.put_object(&self.bucket_name, &key, data).send().await?;
        Ok(key)
    }

    pub async fn delete_object(&self, key: String, username: &String)->Result<(), JoinError>{
        let key= self.get_key(username, key);
        let object = ObjectToDelete::from(key);
        let client = Arc::clone(&self.client);
        let bucket_name = self.bucket_name.clone();
        tokio::task::spawn_blocking(move|| {
            client.delete_object(bucket_name, object);
        }).await?;
        Ok(())

    }

    pub fn get_key(&self, username: &String, key: String)-> String{
        format!("{}/images/{}", username, key)
    }
}

