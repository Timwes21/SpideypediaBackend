use tokio::task::JoinError;
use axum::response::IntoResponse;


#[derive(Debug)]
pub enum UpdateDetailsErrors{
    MongoError,
    TokioJoinErr,
    MinioError
}

impl From<mongodb::error::Error> for UpdateDetailsErrors{
    fn from(value: mongodb::error::Error) -> Self {
        println!("Problem With {value:?}");
        Self::MongoError
    }
}

impl From<JoinError> for UpdateDetailsErrors{
    fn from(value: JoinError) -> Self {
        println!("Problem with Tokio: {value:?}");
        Self::TokioJoinErr
    }
}

impl From<minio::s3::error::Error> for UpdateDetailsErrors{
    fn from(value: minio::s3::error::Error) -> Self {
        println!("Problem with Minio: {value:?}");
        Self::MinioError
    }
}

impl IntoResponse for UpdateDetailsErrors{
    fn into_response(self) -> axum::response::Response {
        "there is an error".into_response()
    }
}



