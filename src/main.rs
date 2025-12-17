mod routes;
mod app_state;
mod mongo_handler;
mod redis_handler;
mod encryption;
mod minio_client;

use axum::{
    Json, Router, 
    extract::{Path, Request, State}, 
    http::{StatusCode, header}, 
    routing::{delete, get, post, put, any}
};
use dotenvy::dotenv;
use app_state::{build_state};
use tower_http::cors::{CorsLayer, Any};
use crate::{
    routes::{
        update_comic_collection::{add_character, add_title, add_volume, delete_character, delete_issue, delete_volume, update_details},
        get_comic_collection::{ handler},
        auth::{create_user, login, logout}
    },
        
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    

    let state = build_state().await;

    let cors_middleware = CorsLayer::new().allow_origin(Any).allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    
    let app: Router = Router::new()
        .route("/delete-char", post(delete_character))//this works
        .route("/update-details", post(update_details))//this works
        .route("/add-character", post(add_character))//this works
        .route("/delete-issue", post(delete_issue))//this works
        .route("/add-title", post(add_title))//this works
        .route("/add-vol", post(add_volume))//this works
        .route("/delete-vol", post(delete_volume))
        .route("/create-user", post(create_user))
        .route("/login", post(login))//this works
        .route("/logout", post(logout))
        .route("/", any(handler))
        .layer(cors_middleware)
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{addr}");
    let request_service = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await;

    match request_service {
        Ok(e) => println!("{:?}", e),
        Err(e) => println!("{:?}", e)

    }
        

}


async fn get_m(){
    println!("Hello");
}



