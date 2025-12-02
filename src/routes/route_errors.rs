use std::collections::HashMap;

use tokio::task::JoinError;
use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse, Json};
use std::error::Error;



#[derive(Debug)]
pub enum RouteError{
    MongoError,
    TokioJoinErr,
    MinioError,
    MultiPartError,
    OptionError,
    UsernameExists,
    LoginError,
    IncorrectPassword,
    CryptError,
    RedisError
}

impl From<mongodb::error::Error> for RouteError{
    fn from(value: mongodb::error::Error) -> Self {
        println!("Problem With {value:?}");
        Self::MongoError
    }
}

impl From<JoinError> for RouteError{
    fn from(value: JoinError) -> Self {
        println!("Problem with Tokio: {value:?}");
        Self::TokioJoinErr
    }
}

impl From<MultipartError> for RouteError{
    fn from(value: MultipartError) -> Self {
        println!("Problem with Multipart: {value:?}");
        Self::MultiPartError
    }
}

impl From<minio::s3::error::Error> for RouteError{
    fn from(value: minio::s3::error::Error) -> Self {
        println!("Problem with Minio: {value:?}");
        Self::MinioError
    }
}

impl From<redis::RedisError> for RouteError{
    fn from(value: redis::RedisError) -> Self {
        println!("Problem with redis: {value:?}");
        Self::RedisError
    }
}



impl IntoResponse for RouteError{
    fn into_response(self) -> axum::response::Response {
        let (status_code, status_message) = match self {
            Self::UsernameExists => (StatusCode::NOT_ACCEPTABLE, "Username Already Exists"),
            Self::IncorrectPassword => (StatusCode::NOT_FOUND, "Password was not correct"),
            _  => (StatusCode::INTERNAL_SERVER_ERROR, "There is an error"),
        };
        let mut message = HashMap::new();
        message.insert("message", status_message);
        (status_code, Json(message)).into_response()
    }
}






