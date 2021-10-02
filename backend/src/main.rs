use axum::{handler::get, response::Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/api", get(handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct User {
    name: String,
}

async fn handler() -> Json<User> {
    Json(User { name: "Keisuke Toyota".to_string() })
}
