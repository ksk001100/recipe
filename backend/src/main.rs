mod auth;
mod errors;
mod handlers;
mod models;

use axum::{
    extract::Extension, handler::get, response::IntoResponse, AddExtensionLayer, Json, Router,
};
use sea_orm::*;
use std::{net::SocketAddr, sync::Arc};
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

pub use models::user::Entity as User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::new(Database::connect(&database_url).await.unwrap());

    let api = Router::new()
        .route("/", get(handler))
        .route("/users", get(handlers::get_users).post(handlers::add_user))
        .route(
            "/users/:id",
            get(handlers::get_user_by_id).delete(handlers::delete_user),
        )
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
    let user = User::find().one(&db).await.unwrap();
    Json(user)
}
