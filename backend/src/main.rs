mod error;
mod models;
mod handlers;

use axum::{
    async_trait,
    body::{Bytes, Full},
    extract::{Extension, FromRequest, RequestParts},
    handler::{get, post, delete},
    http::{self, Response, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use chrono::prelude::*;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, convert::Infallible, fmt::Display, net::SocketAddr, sync::Arc};
use tower_http::add_extension::AddExtensionLayer;
use anyhow::Result;
use sea_orm::*;

pub use models::user::Entity as User;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::new(Database::connect(&database_url).await.unwrap());

    let api = Router::new()
        .route("/", get(handler))
        .route("/users", get(handlers::get_users).post(handlers::add_user))
        .route("/users/:id", get(handlers::get_user_by_id).delete(handlers::delete_user))
        .layer(AddExtensionLayer::new(db))
        .boxed();

    let app = Router::new().nest("/api", api);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler(Extension(db): Extension<Arc<DatabaseConnection>>) -> impl IntoResponse {
    let users = User::find().all(&db).await.unwrap();
    Json(users)
}