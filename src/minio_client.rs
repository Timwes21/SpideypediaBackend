use bytes::Bytes;
use tokio::task::JoinError;
use std::{env, str::FromStr};
use aws_sdk_s3::{self as s3, Client, config::{self, Region, ProvideCredentials, Credentials}, types::{Delete, ObjectIdentifier}};
use aws_smithy_types::byte_stream::ByteStream;
use aws_config::{BehaviorVersion, environment::credentials, load_defaults, meta::credentials::CredentialsProviderChain};


use std::sync::Arc;


#[derive(Clone)]
pub struct MinioClient{
    pub client: Arc<Client>,
    pub bucket_name: String
}

impl MinioClient{
    pub async fn new()-> Self{
        let endpoint = env::var("MINIO_URL").unwrap();
        let key = env::var("MINIO_ACCESS_KEY").unwrap();
        // let bucket_name = env::var("BUCKET_NAME");
        
        

        let credentials = Credentials::new(key.to_string(), key.to_string(), None, None, "railway");


        let behavior_version = BehaviorVersion::latest();
        let config = aws_config::load_defaults(behavior_version).await;
        let client_config = config::Builder::from(&config)
                                .endpoint_url(endpoint)
                                .credentials_provider(credentials)
                                .force_path_style(true)
                                .region(Region::new("us-east-1"))
                                .build();

        
        Self {
            client: Arc::new(Client::from_conf(client_config)),
            bucket_name: "python-test-bucket".to_string()
        }

    }

    pub async fn add_object(&self, bytes: Bytes, image_path: &String)->Result<(), Box<dyn std::error::Error>>{
        let byte_stream = bytes.into();
        
        self.client.put_object()
            .bucket(&self.bucket_name)
            .key(image_path)
            .body(byte_stream)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_object(&self, key: String)->Result<(), Box<dyn std::error::Error>>{
        let res = self.client.delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await;

        if let Err(e) = res{
            println!("there was an error deleting the image");
            println!("{e:?}");
        }
        Ok(())

    }

    pub async fn delete_objects(&self, prefix: String)->Result<(), Box<dyn std::error::Error>>{
        let mut object_ids = Vec::new();
        let prefix = format!("{}/{}", &self.bucket_name, prefix);
        let mut objects = self.client.list_objects_v2()
                                                .bucket(&self.bucket_name)
                                                .prefix(prefix)
                                                .into_paginator()
                                                .send();



        while let Some(e) = objects.next().await{
            let page = e?.contents.unwrap();

            for obj in page {
                let key = obj.key().unwrap();
                let obj_id = ObjectIdentifier::builder()
                                                    .key(key)
                                                    .build()?;
                object_ids.push(obj_id);

            }

            if object_ids.is_empty(){
                return Ok(())
            }

        }
        
        let delete = Delete::builder().set_objects(Some(object_ids)).build()?;

        self.client.delete_objects()
            .bucket(&self.bucket_name)
            .delete(delete).send()
            .await?;
        
        Ok(())
    }

}

