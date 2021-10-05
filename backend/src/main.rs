// use axum::{handler::{get, post}, response::Json, Router};
// use serde::{Serialize, Deserialize};
// use std::net::SocketAddr;
mod database;
mod error;
mod models;

use axum::{
    async_trait,
    body::{Bytes, Full},
    extract::{Extension, FromRequest, RequestParts},
    handler::{get, post},
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

pub use database::DB;
pub use error::Error;
pub use models::User;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let database_url = "mysql://root:password@db/recipe";
    let db = Arc::new(database::connect(database_url).await?);

    let api = Router::new()
        .route("/", get(handler))
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
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

async fn handler(db: Extension<Arc<DB>>) -> Json<Vec<User>> {
    let db: Arc<DB> = db.0;

    let users: Vec<User> =
        sqlx::query_as::<_, User>("SELECT id, name, created_at, updated_at FROM users")
            .fetch_all(db.as_ref())
            .await
            .unwrap();

    Json(users)
}

async fn signin_handler(
    db: Extension<Arc<DB>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let db = db.0;
    let query = format!(
        r#"SELECT * FROM users WHERE email = "{}" AND password_digest = "{}""#,
        payload.email,
        encode_hash(payload.password)
    );
    let exe = sqlx::query_as::<_, User>(&query)
        .fetch_one(db.as_ref())
        .await;

    match exe {
        Ok(_) => Ok(Json(AuthBody {
            token: String::from("token"),
        })),
        Err(_) => Err(AuthError::WrongCredentials),
    }
}

#[derive(Debug, Deserialize)]
struct SignupRequest {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

fn encode_hash(s: String) -> String {
    bcrypt::hash_with(
        BcryptSetup {
            salt: Some("recipe"),
            cost: Some(5),
            variant: Some(BcryptVariant::V2b),
        },
        s,
    )
    .unwrap()
}

async fn signup_handler(
    db: Extension<Arc<DB>>,
    Json(req): Json<SignupRequest>,
) -> impl IntoResponse {
    if req.password != req.password_confirmation {
        return (StatusCode::BAD_REQUEST);
    }

    let mut db = db.0;
    let password = encode_hash(req.password);
    let query = format!(
        r#"INSERT INTO users (name, email, password_digest) VALUES ("{}", "{}", "{}")"#,
        req.name, req.email, password
    );
    sqlx::query(&query).execute(db.as_ref()).await.unwrap();

    (StatusCode::CREATED)
}

impl IntoResponse for AuthError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey<'static>,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret).into_static(),
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthBody {
    token: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    email: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
